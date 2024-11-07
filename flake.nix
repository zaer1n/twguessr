{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    devenv.url = "github:cachix/devenv";
  };
  outputs = { nixpkgs, devenv, ... }@inputs:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
    devShells.${system}.default = devenv.lib.mkShell {
      inherit inputs pkgs;
      modules = [
        {
          languages.rust.enable = true;
          dotenv.enable = true;
          env = {
            LIBCLANG_PATH = pkgs.lib.makeLibraryPath [
              pkgs.llvmPackages.libclang
            ];
            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
              pkgs.vulkan-loader
            ];
          };
          packages = with pkgs; [
            llvmPackages.clang
            llvmPackages.libclang
            cmake
            ffmpeg
            cargo-watch
          ];
          pre-commit.hooks = {
            rustfmt.enable = true;
            clippy.enable = true;
            clippy.settings.denyWarnings = true;
          };
        }
      ];
    };
  };
}