#!/bin/bash

# Script to install Rust and build dependencies for Hyprland Settings

echo "Installing Rust and build dependencies for Hyprland Settings..."

# Install Rust via rustup
if ! command -v rustc &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    echo "Rust is already installed"
fi

# Install system dependencies (for Arch/Manjaro)
if command -v pacman &> /dev/null; then
    echo "Installing GTK4 and Libadwaita dependencies..."
    sudo pacman -S --needed base-devel gtk4 libadwaita

# Install system dependencies (for Debian/Ubuntu)
elif command -v apt-get &> /dev/null; then
    echo "Installing GTK4 and Libadwaita dependencies..."
    sudo apt-get update
    sudo apt-get install -y build-essential libgtk-4-dev libadwaita-1-dev

# Install system dependencies (for Fedora)
elif command -v dnf &> /dev/null; then
    echo "Installing GTK4 and Libadwaita dependencies..."
    sudo dnf install -y gtk4-devel libadwaita-devel

else
    echo "Unknown package manager. Please install GTK4 and Libadwaita manually."
    exit 1
fi

echo ""
echo "Dependencies installed successfully!"
echo "You can now build the application with:"
echo "  cargo build --release"
echo ""
echo "Or use the Makefile:"
echo "  make release"