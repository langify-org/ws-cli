{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs =
    inputs@{
      nixpkgs,
      crane,
      flake-parts,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      perSystem =
        { pkgs, system, ... }:
        let
          craneLib = crane.mkLib pkgs;

          ws = craneLib.buildPackage {
            pname = "ws";
            version = "0.1.0";
            src = builtins.path {
              path = ./.;
              name = "ws-src";
            };
          };
        in
        {
          packages = {
            default = ws;
            ws = ws;
            docs = pkgs.stdenvNoCC.mkDerivation {
              pname = "ws-docs";
              version = "0.1.0";
              src = ./docs;
              nativeBuildInputs = [ pkgs.mdbook ];
              buildPhase = ''
                mdbook build -d $out
                mdbook build ja -d $out/ja
              '';
              dontInstall = true;
            };
          };

          devShells.default = craneLib.devShell {
            packages = with pkgs; [
              cargo-watch
              mdbook
            ];
          };
        };
    };
}
