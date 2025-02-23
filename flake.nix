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
      in
      {
        devShells.default =
          with pkgs;
          mkShell {
            packages = [ rust-bindgen ];
            buildInputs = [
              wayland
              librime
              libxkbcommon
            ];
            nativeBuildInputs = [
              pkg-config
              rustup
              rustPlatform.bindgenHook
            ];

            RIME_INCLUDE_DIR = "${librime}/include";
            RIME_LIB_DIR = "${librime}/lib";
          };
      }
    );
}
