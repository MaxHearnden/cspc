use std::{
    collections::BTreeMap,
    env,
    io::{BufWriter, Write},
    fs::File,
    vec,
};

#[derive (Clone, Debug, serde::Deserialize)]
#[serde(untagged)]
enum CspDirective {
    #[serde(rename = "none")]
    CSPNone,
    Directive(String),
    Prefixed(BTreeMap<String, MaybeVec<CspDirective>>),
}

fn flatten_csp(prefix: &str, directive: &CspDirective) -> Vec<String> {
    match directive {
        CspDirective::Directive(str) => vec![prefix.to_owned() + str],
        CspDirective::Prefixed(directives) => {
            let mut res = vec![];
            for (pfx, subdirectives) in directives {
                for subdirective in subdirectives {
                    let flattened_subdirective = flatten_csp(&(prefix.to_owned() + pfx), subdirective);
                    res.extend_from_slice(&flattened_subdirective);
                }
            }
            res
        }
        CspDirective::CSPNone => vec!["'none'".to_owned()],
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(untagged)]
enum MaybeVec<A> {
    Vector(Vec<A>),
    Singleton(A),
}

enum MaybeVecIter<A> {
    Vector(<Vec<A> as IntoIterator>::IntoIter),
    Singleton(std::iter::Once<A>),
}

impl<A> Iterator for MaybeVecIter<A> {
    type Item = A;
    fn next(&mut self) -> Option<A> {
        match self {
            MaybeVecIter::Vector(v) => v.next(),
            MaybeVecIter::Singleton(s) => s.next(),
        }
    }
}

impl<A> IntoIterator for MaybeVec<A> {
    type Item = A;
    type IntoIter = MaybeVecIter<A>;
    fn into_iter(self) -> Self::IntoIter {
        match self {
            MaybeVec::Vector(v) => MaybeVecIter::Vector(v.into_iter()),
            MaybeVec::Singleton(s) => MaybeVecIter::Singleton(std::iter::once(s)),
        }
    }
}

impl<A> Into<Vec<A>> for MaybeVec<A> {
    fn into(self) -> Vec<A> {
        match self {
            MaybeVec::Vector(v) => v,
            MaybeVec::Singleton(a) => vec![a],
        }
    }
}

#[derive (Clone, Debug, serde::Deserialize)]
struct CSP {
    #[serde(flatten)]
    directives: BTreeMap<String, MaybeVec<CspDirective>>,
}

fn main() {
    let mut args = env::args();
    args.next();
    let path = args.next().unwrap_or("/dev/stdin".to_owned());
    let output_path = args.next().unwrap_or("/dev/stdout".to_owned());
    let mut output = BufWriter::new(File::create(output_path).expect("Failed to open output file"));
    let file = File::open(path).expect("Failed to open file");
    let csp: CSP = serde_yml::from_reader(file).expect("Failed to read CSP");
    dbg!(&csp);
    for (name, contents) in csp.directives {
        write!(output, "{name}").expect("Failed to write CSP");
        for directive in contents {
            for flattened_directive in flatten_csp("", &directive) {
                write!(output, " {flattened_directive}").expect("Failed to write CSP");
            }
        }
        write!(output, "; ").expect("Failed to write CSP");
    }
    output.flush().expect("Failed to write CSP");
}
