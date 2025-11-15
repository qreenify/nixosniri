# Project Context for Claude Code

## Directory Structure

This workspace (`~/claude/`) is for **projects and general development work**.

**NixOS configuration files are NOT here** - they live in `~/.config/nixos/`

### Structure Overview

```
~/.config/nixos/          # NixOS system configuration (separate git repo)
├── flake.nix
├── modules/              # System modules (boot, networking, packages, etc.)
├── config/               # Application configs (niri, waybar, fuzzel)
├── scripts/              # System scripts (audio, mic controls)
├── deploy.sh             # Deploy configs to /etc/nixos
└── rebuild.sh            # Deploy + rebuild NixOS

~/claude/                 # This directory - projects & development
├── .claude/              # Claude Code context (this file)
├── projects/             # Development projects
│   └── settings_app/     # GTK settings application
└── (other projects)
```

## System Information

### Shell
- **User uses Nushell** - NOT bash
- Command chaining: Use `;` instead of `&&`
  - ✅ Correct: `pkill waybar; hyprctl dispatch exec waybar`
  - ❌ Wrong: `pkill waybar && hyprctl dispatch exec waybar`

### Hardware
- **GPU**: NVIDIA GeForce RTX 4080
- **Monitors**: 4 displays
  - DP-2: 2560x1440@155Hz (main monitor, workspaces 1-6)
  - HDMI-A-1: 1920x1080@60Hz (top, workspace 8)
  - DP-1: 2560x1440@60Hz (right, workspace 9, vertical)
  - DP-3: 2560x1440@60Hz (left, workspace 7, vertical)

### Desktop Environment
- **Compositor**: Hyprland (primary), Niri (available but not actively used)
- **Display Manager**: Ly (TUI greeter)
- **Bar**: Waybar (started by Hyprland, NOT systemd)
- **Launcher**: Fuzzel
- **Terminal**: Alacritty
- **Cursor**: macOS-BigSur (apple-cursor package)

### Critical NixOS Behavior
- **Config changes require rebuild**: `rebuild` command
- **Changes take effect**: After rebuild AND logout/login (or reboot for kernel/boot changes)
- **Waybar config changes**: Require rebuild, then waybar reload
- **NEVER use `&&` for commands** - user has Nushell, use `;` instead

### ⚠️ CRITICAL: Home-manager File Conflicts
**ALWAYS use `force = true` for ALL xdg.configFile and home.file declarations!**

This is **NOT optional** - it prevents "file would be clobbered" errors on rebuild.
  ```nix
  # For files in ~/.config/ - use xdg.configFile:
  xdg.configFile."somedir" = {
    source = ../path;
    recursive = true;
    force = true;  # Prevents "file would be clobbered" errors
  };

  # For files elsewhere in $HOME - use home.file:
  home.file.".local/share/somedir" = {
    source = ../path;
    recursive = true;
    force = true;
  };
  ```
  - Especially important for theme files, configs that get modified, or any files that may exist from previous runs
  - `xdg.configFile."path"` → deploys to `~/.config/path`
  - `home.file.".some/path"` → deploys to `~/.some/path`

## Important Rules

### When Working with NixOS Configs

**ALWAYS edit files in `~/.config/nixos/`** when the user wants to:
- Add/remove system packages
- Modify system settings
- Change boot configuration
- Update desktop environment settings
- Modify niri/waybar/fuzzel configs
- Update any NixOS module

**NEVER** edit NixOS config files in `~/claude/` - they don't exist here anymore.

**ALWAYS remind user to rebuild after config changes**: `rebuild`

### Workflow for NixOS Changes

```bash
# Edit configs in ~/.config/nixos/
cd ~/.config/nixos
vim modules/packages.nix

# Apply changes
rebuild              # Command available system-wide
# or
./rebuild.sh
```

### When Working in ~/claude

This directory is for:
- Development projects (in `projects/`)
- Non-NixOS configuration
- General development work
- Project-specific scripts/tools

## Key Commands

- `rebuild` - Deploy and rebuild NixOS (works from anywhere)
- `rebuild dry-build` - Preview what would change
- `rebuild test` - Test config without boot entry

## Git Repositories

- `~/.config/nixos/.git` - NixOS configuration repo (synced to GitHub)
- `~/claude/.git` - Projects and development repo (auto-backup every hour)

These are **separate repositories** - don't confuse them.

### NixOS Config GitHub Sync

The `~/.config/nixos/` directory is set up with another Claude session for continuous GitHub sync.

**Important**: When editing NixOS configs, changes will be automatically committed and pushed to GitHub. This ensures:
- Version control for all system configurations
- Easy rollback if needed
- Backup of the entire NixOS setup

## Dual Boot Setup

### Windows Bootloader (Manual Setup)

The system uses **systemd-boot** which can only see bootloaders on its own EFI partition.

**Setup (done once, persists across NixOS rebuilds):**

1. Windows EFI partition: `/dev/nvme2n1p3` (labeled "SYSTEM")
2. NixOS EFI partition: `/dev/nvme1n1p1` (mounted at `/boot`)

To make Windows appear in systemd-boot menu:

```bash
# Mount Windows EFI partition
sudo mkdir -p /mnt/win-efi
sudo mount /dev/nvme2n1p3 /mnt/win-efi

# Copy Windows bootloader to NixOS EFI partition
sudo mkdir -p /boot/EFI/Microsoft
sudo cp -r /mnt/win-efi/EFI/Microsoft/Boot /boot/EFI/Microsoft/

# Unmount
sudo umount /mnt/win-efi
```

**Why this is safe for reproducibility:**
- NixOS only manages `/boot/loader/` and `/boot/EFI/Linux/`
- Windows files in `/boot/EFI/Microsoft/` are ignored by NixOS
- These files persist across all NixOS rebuilds
- Only need to recopy if `/boot` partition is completely wiped

**Note:** If you ever recreate the `/boot` partition from scratch, you'll need to repeat this copy step.

## Omarchy Theme System

### Overview
The system uses **Omarchy theming** for consistent visual appearance across all applications.

- **Theme location**: `~/.config/omarchy/themes/`
- **Current theme symlink**: `~/.config/omarchy/current/theme`
- **Theme switcher**: `theme <theme-name>` command
- **Wallpaper selector**: `wallpaper` command (with image preview)

### Supported Applications

**Live theming (instant reload):**
- ✅ Waybar (CSS generated dynamically)
- ✅ Hyprland (border colors)
- ✅ Mako notifications
- ✅ Walker launcher
- ✅ Alacritty terminal (copies theme file for live reload)
- ✅ Kitty terminal (SIGUSR1 signal)
- ✅ Ghostty terminal (SIGUSR2 signal)
- ✅ Btop system monitor
- ✅ Wallpaper via swaybg

**Manual theming:**
- 📝 Vesktop/Discord (generates custom CSS)
- 📝 Browsers (no standard API - GTK theme only)

### Theme Script Details

**Location**: `~/.config/nixos/scripts/omarchy-theme-set`

**Key features:**
- Generates waybar CSS from theme colors + static rules
- Copies Alacritty theme to avoid symlink detection bug (#5852)
- Sends reload signals to running applications
- Sets wallpaper (first image in backgrounds/ directory)

**Important implementation notes:**
- Waybar style.css is NOT managed by home-manager (dynamically generated)
- Alacritty theme.toml is created by activation script, updated by theme script
- Browser theming disabled (requires omarchy-chromium fork, not in nixpkgs)

### Wallpaper Selector

**Command**: `wallpaper [theme-name]`

**Features:**
- Interactive fzf-based selection
- **Pixel-perfect preview** in Kitty/Ghostty terminals (Kitty graphics protocol)
- Text-based preview in other terminals (chafa)
- Auto-detects current theme if no argument provided

**Usage:**
```bash
wallpaper              # Select from current theme
wallpaper catppuccin   # Select from specific theme
kitty -e wallpaper     # Force pixel-perfect preview
```

## Terminal Preferences

### Installed Terminals
1. **Alacritty** - Primary daily driver, fastest startup
2. **Kitty** - For pixel-perfect image previews
3. **Ghostty** - Modern alternative with Kitty protocol support

### User Preferences
- **Primary terminal**: Alacritty (minimal, fast)
- **Image work**: Kitty or Ghostty (pixel-perfect wallpaper previews)
- **Shell**: Nushell (use `;` for command chaining, not `&&`)

## Recent Session Work (2025-11-15)

### Theme System Fixes
1. **Waybar theming** - Fixed gray waybar issue:
   - Root cause: Theme files only had color variables, no CSS rules
   - Solution: Created `generate-waybar-style` script to combine colors + rules
   - Waybar style.css now dynamically generated, not symlinked

2. **Alacritty live reload** - Fixed theme not changing in open terminals:
   - Root cause: Alacritty bug #5852 - imported symlinks don't trigger reload
   - Solution: Copy theme to real file instead of symlink
   - Uses home.activation script for initial file creation

3. **Theme script hanging** - Fixed infinite hang on browser theming:
   - Root cause: `wait` command waiting for browser processes that never exit
   - Solution: Removed `wait`, browsers disabled (not supported without omarchy-chromium)

4. **Wallpaper selector** - Created interactive preview tool:
   - Uses fzf for selection
   - Pixel-perfect preview in Kitty/Ghostty
   - Fallback to chafa in other terminals
   - Auto-detects current theme

### Packages Added
- `chafa` - Terminal image viewer
- `kitty` - GPU-accelerated terminal with image protocol
- `ghostty` - Modern terminal with Kitty protocol support
- `playerctl` - Media player control for waybar mpris module

### Scripts Created/Modified
- `omarchy-theme-set` - Theme switcher (fixed hanging, removed browser theming)
- `generate-waybar-style` - Generates waybar CSS from theme colors
- `omarchy-wallpaper-select` - Interactive wallpaper selector with preview
- `mic-rgb-sync` - Fixed to use wpctl instead of pactl

### Configuration Changes
- Fixed double-slash in theme symlink paths
- Added Alacritty theme initialization via home.activation
- Removed waybar style.css from home-manager (dynamically generated now)
- Added shell alias: `wallpaper` for wallpaper selector
- Added Kitty and Ghostty terminal configurations with theme support
- **Lesson learned**: ALWAYS add `force = true` to xdg.configFile/home.file (made more prominent in context)
