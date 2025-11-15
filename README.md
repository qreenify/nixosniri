# Development Workspace

This directory (`~/claude/`) is for **development projects and general work**.

> **Note**: NixOS system configuration lives in `~/.config/nixos/` (separate git repo with GitHub sync)

## Directory Structure

```
~/claude/
├── .claude/              # Claude Code context and settings
│   └── context.md        # System info, preferences, recent work
├── projects/             # Development projects
│   └── settings_app/     # GTK settings application for Hyprland/Niri
└── README.md             # This file

~/.config/nixos/          # NixOS configuration (separate repo, GitHub synced)
├── flake.nix             # Nix flake configuration
├── modules/              # System modules (boot, networking, packages, home-manager)
├── config/               # App configs (hyprland, waybar, fuzzel)
├── scripts/              # System scripts (theme, wallpaper, audio controls)
└── omarchy/              # Theme system (themes, config templates)
```

## System Setup

### Hardware
- **GPU**: NVIDIA GeForce RTX 4080
- **Monitors**: 4 displays (155Hz main + 3x 60Hz)
- **Shell**: Nushell
- **Compositor**: Hyprland (primary)

### Key Applications
- **Terminal**: Alacritty (primary), Kitty/Ghostty (image support)
- **Bar**: Waybar (dynamic theme generation)
- **Launcher**: Fuzzel
- **Notifications**: Mako

## Theming System

The system uses **Omarchy** for unified theming across all applications.

### Quick Commands

```bash
# Theme switching
theme tokyo-night        # Switch to tokyo-night theme
theme catppuccin        # Switch to catppuccin theme

# Wallpaper selection (with preview)
wallpaper               # Select from current theme
wallpaper catppuccin    # Select from specific theme
kitty -e wallpaper      # Get pixel-perfect preview

# System management
rebuild                 # Deploy and rebuild NixOS
rebuild dry-build       # Preview what would change
```

### Supported Applications

**Live theming (instant reload):**
- ✅ Waybar, Hyprland, Mako, Walker
- ✅ Alacritty, Kitty, Ghostty, Btop
- ✅ Wallpaper (swaybg)

**Generated theming:**
- 📝 Vesktop/Discord (custom CSS)

### How It Works

1. **Theme files**: `~/.config/omarchy/themes/<theme-name>/`
   - `waybar.css` - Color variables
   - `hyprland.conf` - Border colors
   - `alacritty.toml` - Terminal colors
   - `backgrounds/` - Wallpaper images

2. **Theme switcher**: `~/.script/theme <theme-name>`
   - Updates symlink: `~/.config/omarchy/current/theme`
   - Generates waybar CSS (colors + rules)
   - Copies Alacritty theme (avoids symlink bug)
   - Sends reload signals to running apps
   - Sets wallpaper

3. **Wallpaper selector**: `~/.script/omarchy-wallpaper-select`
   - Interactive fzf-based selection
   - Pixel-perfect preview in Kitty/Ghostty
   - Fallback to chafa in other terminals

## Git Repositories

### ~/claude (This Directory)
- **Purpose**: Development projects
- **Git**: Auto-backup every hour to GitHub
- **Branch**: main

### ~/.config/nixos
- **Purpose**: NixOS system configuration
- **Git**: Continuous sync to GitHub (managed by separate Claude session)
- **Important**: All NixOS changes are version controlled

## NixOS Configuration

### Editing System Config

```bash
cd ~/.config/nixos

# Edit any module
vim modules/packages.nix      # Add/remove packages
vim modules/home.nix          # User environment config
vim config/hypr/hyprland.conf # Hyprland settings

# Apply changes (from anywhere)
rebuild
```

### Important Notes

- **Shell**: User has Nushell - use `;` instead of `&&` for command chaining
- **Home-manager**: Use `force = true` for files that might exist
- **Changes take effect**: After rebuild + logout/login (or reboot for kernel changes)

## Recent Major Work

### Session 2025-11-15: Theme System Overhaul

**Problems Fixed:**
1. ❌ Waybar stayed gray when switching themes
2. ❌ Alacritty didn't live-reload themes in open terminals
3. ❌ Theme script hung infinitely on browser theming
4. ❌ Mic RGB sync stopped working
5. ❌ No way to choose wallpaper (always picked first one)

**Solutions Implemented:**
1. ✅ Created `generate-waybar-style` - combines theme colors with CSS rules
2. ✅ Fixed Alacritty live reload - copies theme file instead of symlink
3. ✅ Removed browser theming (requires omarchy-chromium fork, not available)
4. ✅ Fixed mic-rgb-sync to use wpctl instead of pactl
5. ✅ Created interactive wallpaper selector with image preview

**New Features:**
- 🎨 Pixel-perfect wallpaper previews in Kitty/Ghostty terminals
- 🖼️ Interactive wallpaper selection with fzf
- 🎯 Live theme reload in all open Alacritty windows
- 🔧 Dynamically generated waybar styling

## Projects

- **settings_app**: GTK4 settings application for Wayland compositors (Hyprland/Niri)

## Documentation

- `.claude/context.md` - Comprehensive system context for Claude Code sessions
- `~/.config/nixos/README.md` - NixOS configuration documentation (to be created)

## User Preferences

- **Terminal**: Alacritty for daily use, Kitty/Ghostty for image work
- **Theme**: Omarchy system with live reload
- **Workflow**: Make changes in `~/.config/nixos/`, rebuild, test
- **Backup**: Both repos auto-sync to GitHub
