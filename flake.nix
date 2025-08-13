{
  description = "wayime";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        toolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
          ];
        };
        # rime related environemnt variables
        rimeEnvs = with pkgs; {
          RIME_INCLUDE_DIR = "${librime}/include";
          RIME_LIB_DIR = "${librime}/lib";
        };
        buildInputs = with pkgs; [
          wayland
          librime
          libxkbcommon
        ];
        nativeBuildInputs = with pkgs; [
          pkg-config
          toolchain
          rustPlatform.bindgenHook
        ];
      in
      {

        # 打包
        packages.default =
          with pkgs;
          rustPlatform.buildRustPackage (
            {
              pname = "wayime";
              version = "0.1.0";
              src = ./.;
              cargoLock = {
                lockFile = ./Cargo.lock;
              };

              inherit buildInputs nativeBuildInputs;
            }
            // rimeEnvs
          );

        # 开发环境
        devShells.default =
          with pkgs;
          mkShell (
            {
              inherit buildInputs nativeBuildInputs;

              # In case path contains space
              shellHook = ''
                export NIX_LDFLAGS="''${NIX_LDFLAGS/-rpath $out\/lib /}"
              '';
            }
            // rimeEnvs
          );
      }
    );
}
