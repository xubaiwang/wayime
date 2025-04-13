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
        makeCommon =
          with pkgs;
          {
            devShell ? false,
            rimeDataPkg ? rime-data,
          }:
          {
            packages = [
              rust-analyzer
              clippy
              rustfmt
            ];
            buildInputs = [
              wayland
              librime
              libxkbcommon
              rimeDataPkg
            ];
            nativeBuildInputs = [
              pkg-config
              rustc
              cargo
              rustPlatform.bindgenHook
            ] ++ lib.optionals devShell [ rustup ];
            RIME_INCLUDE_DIR = "${librime}/include";
            RIME_LIB_DIR = "${librime}/lib";
            RIME_SHARED_DATA_DIR = "${rimeDataPkg}/share/rime-data";
          };
      in
      {
        # 包
        packages.default =
          with pkgs;
          rustPlatform.buildRustPackage (
            {
              pname = "wlrime";
              version = "0.1.0";
              src = ./.;
              cargoLock = {
                lockFile = ./Cargo.lock;
                outputHashes = {
                  "rime-api-0.1.0" = "sha256-VUhvKzC6sgPJidQ9bMLJvu3xZYqkThvGzzVsJUqn0rw=";
                };
              };
            }
            // makeCommon { }
          );

        # 開發環境
        devShells.default =
          with pkgs;
          mkShell (makeCommon {
            devShell = true;
          });
        # shellHook = ''
        #   export NIX_LDFLAGS="''${NIX_LDFLAGS/-rpath $out\/lib /}"
        # ''
      }
    );
}
