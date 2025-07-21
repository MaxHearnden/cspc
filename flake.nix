{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs = { nixpkgs, self }:
  let inherit (nixpkgs) legacyPackages lib;
  in {
    devShells = lib.mapAttrs (system: pkgs: {
      default = self.packages.${system}.default.overrideAttrs (
        { nativeBuildInputs ? [], ... }: {
          nativeBuildInputs = nativeBuildInputs ++ [ pkgs.cargo-watch ];
          src = null;
        });
    }) legacyPackages;
    packages = lib.mapAttrs (system: pkgs: {
      default = pkgs.callPackage ./package.nix {};
    }) legacyPackages;
  };
}
