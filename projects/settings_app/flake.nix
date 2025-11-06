{
  description = "Hyprland Settings - A modern settings app for Hyprland";

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

        nativeBuildInputs = with pkgs; [
          rustToolchain
          pkg-config
          wrapGAppsHook4
          gobject-introspection
        ];

        buildInputs = with pkgs; [
          gtk4
          libadwaita
          glib
          cairo
          pango
          gdk-pixbuf
          graphene
        ];

        hyprland-settings = pkgs.rustPlatform.buildRustPackage {
          pname = "hyprland-settings";
          version = "0.1.0";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          inherit nativeBuildInputs buildInputs;

          postInstall = ''
            install -Dm644 assets/hyprland-settings.desktop $out/share/applications/hyprland-settings.desktop
          '';
        };

      in
      {
        packages = {
          default = hyprland-settings;
          hyprland-settings = hyprland-settings;
        };

        devShells.default = pkgs.mkShell {
          inherit buildInputs;

          nativeBuildInputs = nativeBuildInputs ++ (with pkgs; [
            # Additional dev tools
            rustfmt
            clippy
            cargo-watch
            glib.dev
          ]);

          shellHook = ''
            echo "Hyprland Settings development environment"
            echo ""
            echo "Available commands:"
            echo "  cargo build    - Build the project"
            echo "  cargo run      - Run the application"
            echo "  cargo watch    - Watch for changes and rebuild"
            echo ""
            export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath buildInputs}:$LD_LIBRARY_PATH
            export GIO_MODULE_DIR="${pkgs.glib-networking}/lib/gio/modules/"
            export XDG_DATA_DIRS="${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk4}/share/gsettings-schemas/${pkgs.gtk4.name}:$XDG_DATA_DIRS"
          '';
        };

        apps.default = flake-utils.lib.mkApp {
          drv = hyprland-settings;
        };
      });
}