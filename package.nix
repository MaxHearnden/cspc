{ nix-gitignore, rustPlatform }:

rustPlatform.buildRustPackage {
  pname = "cspc";
  version = "0.1.0";

  src = builtins.path {
    path = ./.;
    name = "source";
    filter = nix-gitignore.gitignoreFilterSourcePure (_: _: true) [
      "*.nix"
      "flake.lock"
    ] ./.;
  };
  
  cargoLock.lockFile = ./Cargo.lock;
}
