{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      crane,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
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
        };

        devShells.default = craneLib.devShell {
          packages = with pkgs; [
            cargo-watch
          ];
        };
      }
    );
}
