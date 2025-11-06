#!/usr/bin/env bash

# Run script for Hyprland Settings on NixOS

echo "Starting Hyprland Settings..."

# Enter nix-shell and run the app
nix-shell --run "cargo run"