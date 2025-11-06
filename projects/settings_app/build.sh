#!/usr/bin/env bash

# Build script for Hyprland Settings on NixOS

echo "Building Hyprland Settings..."

# Build in release mode for better performance
nix-shell --run "cargo build --release"

echo ""
echo "Build complete! Run the app with:"
echo "  ./run.sh"
echo "Or directly:"
echo "  ./target/release/hyprland-settings"