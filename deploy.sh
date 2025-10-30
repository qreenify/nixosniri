#!/usr/bin/env bash
# Deploy NixOS configuration from ~/claude to /etc/nixos

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TARGET="/etc/nixos"

echo "🚀 Deploying NixOS configuration..."
echo "   Source: $SCRIPT_DIR"
echo "   Target: $TARGET"
echo

# Copy main flake
echo "📝 Copying flake.nix..."
sudo cp "$SCRIPT_DIR/flake.nix" "$TARGET/"

# Copy modules directory
echo "📦 Copying modules/..."
sudo cp -r "$SCRIPT_DIR/modules" "$TARGET/"

# Copy config directory
echo "⚙️  Copying config/..."
sudo cp -r "$SCRIPT_DIR/config" "$TARGET/"

# Copy scripts directory (optional)
if [ -d "$SCRIPT_DIR/scripts" ]; then
    echo "📜 Copying scripts/..."
    sudo cp -r "$SCRIPT_DIR/scripts" "$TARGET/"
fi

echo
echo "✅ Deployment complete!"
echo
echo "Next steps:"
echo "  1. cd /etc/nixos"
echo "  2. sudo nix flake update  (if you want to update dependencies)"
echo "  3. sudo nixos-rebuild switch --flake /etc/nixos#nixos"
echo
echo "Or just run: rebuild"
