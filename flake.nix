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
        common = with pkgs; {
          buildInputs = [
            wayland
            librime
            libxkbcommon
          ];
          RIME_INCLUDE_DIR = "${librime}/include";
          RIME_LIB_DIR = "${librime}/lib";
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
              nativeBuildInputs = [
                pkg-config
                rustPlatform.bindgenHook
              ];
            }
            // common
          );

        # 開發環境
        devShells.default =
          with pkgs;
          mkShell (
            {
              nativeBuildInputs = [
                pkg-config
                rustup
                rustPlatform.bindgenHook
              ];
            }
            // common
          );
      } 
    );
}
