{
  description = "Flake utils demo";

  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        # rime related environemnt variables
        rimeEnvs = with pkgs; {
          RIME_INCLUDE_DIR = "${librime}/include";
          RIME_LIB_DIR = "${librime}/lib";
          RIME_SHARED_DATA_DIR = "${rime-data}/share/rime-data";
        };
        buildInputs = with pkgs; [
          wayland
          librime
          libxkbcommon
          rime-data
        ];
        nativeBuildInputs = with pkgs; [
          pkg-config
          rustc
          cargo
          rustPlatform.bindgenHook
        ];
      in
      {

        # 打包
        packages.default =
          with pkgs;
          rustPlatform.buildRustPackage (
            {
              pname = "wlrime";
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
              packages = [
                rust-analyzer
                clippy
                rustfmt
              ];

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
