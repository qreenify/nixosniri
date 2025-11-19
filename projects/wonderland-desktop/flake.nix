{
  description = "Wonderland Desktop - Rust desktop environment components";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust toolchain
            rustToolchain
            pkg-config

            # GUI dependencies
            gtk4
            libadwaita
            glib
            cairo
            pango
            gdk-pixbuf

            # Wayland
            wayland
            wayland-protocols
            libxkbcommon

            # Audio
            pipewire

            # Build tools
            cmake
            ninja
          ];

          shellHook = ''
            echo "Wonderland Desktop development environment"
            echo "Run 'cargo build' to build all crates"
          '';

          # For iced/wayland
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
            pkgs.wayland
            pkgs.libxkbcommon
            pkgs.vulkan-loader
          ];
        };
      }
    );
}
