{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        defaultPackage = naersk-lib.buildPackage ./.;
        devShell = with pkgs; mkShell rec {
          buildInputs = pkgs.lib.optionals pkgs.stdenv.isLinux (with pkgs; [
            # Rust deps
            cargo
            rustc
            rustfmt
            pre-commit
            rustPackages.clippy
            rust-analyzer

            # Audio
            alsa-lib

            # Controller support
            udev

            # Graphics (generic)
            vulkan-loader

            # Graphics (Wayland)
            libxkbcommon
            wayland

            # Graphics (X11)
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr

            # GTK
            gtk3-x11.dev
          ]);

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;

          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      });
}
