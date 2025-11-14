# Development Workspace

This directory is for **development projects and general work**.

## Directory Structure

```
~/claude/
├── .claude/              # Claude Code context and settings
├── projects/             # Development projects
│   └── settings_app/     # GTK settings application for Hyprland/Niri
└── README.md             # This file
```

## NixOS Configuration

**NixOS configuration is NOT in this directory.**

NixOS configs are located at: `~/.config/nixos/`

To edit NixOS system configuration:
```bash
cd ~/.config/nixos
# Edit any config files
vim modules/packages.nix

# Apply changes
rebuild              # Works from anywhere
```

See `~/.config/nixos/README.md` for NixOS documentation.

## Projects

- **settings_app**: GTK4 settings application for Wayland compositors (Hyprland/Niri)

## Claude Code Context

Instructions for Claude Code are stored in `.claude/context.md` to maintain consistency across sessions.
