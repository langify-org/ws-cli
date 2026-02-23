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
      flake = {
        homeManagerModules.default = import ./nix/hm-module.nix;
      };

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

          src = builtins.path {
            path = ./.;
            name = "ws-src";
          };

          commonArgs = {
            pname = "ws";
            version = "0.4.0";
            inherit src;
          };

          # 依存クレートのビルド成果物をキャッシュ
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;

          # メインパッケージ（テストなし）
          ws = craneLib.buildPackage (commonArgs // {
            inherit cargoArtifacts;
            doCheck = false;
          });
        in
        {
          packages = {
            default = ws;
            ws = ws;
            docs = pkgs.stdenvNoCC.mkDerivation {
              pname = "ws-docs";
              version = "0.4.0";
              src = ./docs;
              nativeBuildInputs = [ pkgs.mdbook ];
              buildPhase = ''
                mdbook build -d $out
                mdbook build ja -d $out/ja
              '';
              dontInstall = true;
            };
          };

          # nix flake check で実行
          checks = {
            ws-test = craneLib.cargoTest (commonArgs // {
              inherit cargoArtifacts;
              nativeCheckInputs = [ pkgs.git ];
            });
            ws-clippy = craneLib.cargoClippy (commonArgs // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- -D warnings";
            });
            ws-fmt = craneLib.cargoFmt { inherit src; };
          };

          devShells.default = craneLib.devShell {
            packages = with pkgs; [
              cargo-watch
              mdbook
              rust-analyzer
              rustPlatform.rustLibSrc
            ];

            RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
          };
        };
    };
}
