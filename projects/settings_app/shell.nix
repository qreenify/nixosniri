{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    # Rust toolchain
    rustc
    cargo
    rustfmt
    clippy
    rust-analyzer

    # Build tools
    pkg-config
    wrapGAppsHook4
    gobject-introspection

    # Optional dev tools
    cargo-watch
  ];

  buildInputs = with pkgs; [
    # GTK4 and Adwaita
    gtk4
    libadwaita

    # Core libraries
    glib
    cairo
    pango
    gdk-pixbuf
    graphene

    # Additional GTK dependencies
    glib-networking
    gsettings-desktop-schemas
  ];

  shellHook = ''
    echo "Hyprland Settings development environment (NixOS)"
    echo ""
    echo "Building and running:"
    echo "  cargo build           - Build the project"
    echo "  cargo run            - Run the application"
    echo "  cargo build --release - Build optimized version"
    echo ""

    # Set up GTK environment variables
    export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath [
      pkgs.gtk4
      pkgs.libadwaita
      pkgs.cairo
      pkgs.pango
      pkgs.glib
      pkgs.gdk-pixbuf
      pkgs.graphene
    ]}:$LD_LIBRARY_PATH"

    export GIO_MODULE_DIR="${pkgs.glib-networking}/lib/gio/modules/"
    export XDG_DATA_DIRS="${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk4}/share/gsettings-schemas/${pkgs.gtk4.name}:$XDG_DATA_DIRS"

    # Ensure GSettings schemas are available
    export GSETTINGS_SCHEMA_DIR="${pkgs.glib.getSchemaPath pkgs.gtk4}:${pkgs.glib.getSchemaPath pkgs.gsettings-desktop-schemas}"
  '';
}