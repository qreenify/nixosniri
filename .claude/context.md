# Project Context for Claude Code

## Directory Structure

This workspace (`~/claude/`) is for **projects and general development work**.

**NixOS configuration files are NOT here** - they live in `~/.config/nixos/`

### Structure Overview

```
~/.config/nixos/          # NixOS system configuration (separate git repo)
├── flake.nix
├── modules/              # System modules (boot, networking, packages, etc.)
│   ├── base/             # Base system (packages, shell, desktop, etc.)
│   ├── optional/         # Optional modules
│   └── personal/         # Personal additions
├── config/               # Application configs (hypr, waybar, rofi, ghostty, etc.)
├── scripts/              # System scripts (theme, audio, mic controls)
├── tools/                # Build tools (rebuild.sh, deploy.sh, vm-test.sh)
└── theme/                # Theme system (themes, configs, scripts)

~/claude/                 # This directory - projects & development
├── .claude/              # Claude Code context (this file)
└── projects/             # Development projects
```

## System Architecture

### NixOS + Home-Manager Setup

**CRITICAL**: This system uses **both NixOS and Home-Manager**:
- **NixOS modules** (`modules/base/*.nix`) manage system-level packages and services
- **Home-Manager** manages user configs, dotfiles, and theme files
- Package changes go in `/home/qreenify/.config/nixos/modules/base/packages.nix` or `modules/personal/packages.nix`
- User configs are managed via home-manager in the modules (NOT manually editing dotfiles)

### Shell & Environment
- **Shell**: Fish (configured via home-manager in `modules/base/shell.nix`)
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
- **Launcher**: Rofi (with plugins: rofi-calc, rofi-file-browser, rofimoji)
- **Terminals**:
  - **Ghostty** - Primary terminal (fastest startup, daily driver)
  - **Kitty** - For pixel-perfect image previews (Kitty graphics protocol)
  - **Alacritty** - Minimal alternative (also installed)
- **Browser**: Zen (primary), Brave (installed)
- **File Manager**: Nautilus (GNOME file manager with GVFS network support)
- **Image Viewer**: imv (minimal, themeable)
- **PDF Viewer**: Zathura (minimal, vim-like, themeable)
- **Video Player**: mpv

### Critical NixOS Behavior
- **Config changes require rebuild**: `rebuild` command
- **Changes take effect**: After rebuild AND logout/login (or reboot for kernel/boot changes)
- **Waybar config changes**: Require rebuild, then waybar reload
- **Package management**: Edit `.nix` files in `modules/`, then rebuild

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

**Why this matters**:
- Home-manager manages user dotfiles/configs
- Without `force = true`, rebuilds fail if files already exist
- Especially important for theme files, configs that get modified, or any files that may exist from previous runs
- `xdg.configFile."path"` → deploys to `~/.config/path`
- `home.file.".some/path"` → deploys to `~/.some/path`

## Important Rules

### When Working with NixOS Configs

**ALWAYS edit files in `~/.config/nixos/`** when the user wants to:
- Add/remove system packages → `modules/base/packages.nix` or `modules/personal/packages.nix`
- Install new applications → Add to `environment.systemPackages` in packages.nix
- Modify system settings → Edit appropriate module in `modules/`
- Change boot configuration → `modules/base/boot.nix`
- Update desktop environment settings → `modules/base/desktop.nix` or `modules/base/hyprland.nix`
- Modify application configs (hypr/waybar/rofi/niri) → Edit in `config/` directory
- Update any NixOS or home-manager module

**NEVER** edit NixOS config files in `~/claude/` - they don't exist here.

**NEVER** manually edit dotfiles in `~/.config/` - they are managed by home-manager and will be overwritten on rebuild.

**ALWAYS remind user to rebuild after config changes**: `rebuild`

### Workflow for NixOS Changes

```bash
# 1. Edit configs in ~/.config/nixos/
cd ~/.config/nixos
nvim modules/base/packages.nix  # Add packages here

# 2. Apply changes
rebuild              # Command available system-wide
# or
./tools/rebuild.sh

# 3. For some changes, logout/login may be required
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
- `theme <theme-name>` - Switch theme (live reload supported apps)
- `wallpaper` - Interactive wallpaper selector with fzf preview
- `theme-sync` - Download/update themes from external source

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

## Wonderland Theme System

### Overview

The system uses **Wonderland theming** for consistent visual appearance across all applications.

- **Theme location**: `~/.config/nixos/theme/themes/` (in NixOS config) and `~/.config/theme/themes/`
- **Current theme symlink**: `~/.config/theme/current/theme`
- **Theme switcher**: `theme <theme-name>` command (in `~/.script/theme`)
- **Wallpaper selector**: `wallpaper` command (with image preview)

### How Theming Works

**Theme files are deployed via home-manager**, not manually copied:
- Base theme always deployed via NixOS (in `modules/base/theming.nix`)
- Additional themes synced via `theme-sync` script (stored in `~/.config/nixos/theme/themes/`)
- Theme script (`~/.script/theme`) generates configs and reloads applications
- Each theme directory contains: `colors`, `waybar.css`, `rofi.rasi`, `ghostty.conf`, `alacritty.toml`, `kitty.conf`, `hypr.conf`, etc.

### Supported Applications

**Live theming (instant reload):**
- ✅ Waybar (CSS generated dynamically from theme colors + static rules)
- ✅ Hyprland (border colors updated via hyprctl)
- ✅ Mako notifications (reloaded with new colors)
- ✅ Rofi launcher (theme file generated dynamically, no restart needed)
- ✅ Ghostty terminal (theme file copied, SIGUSR2 signal sent)
- ✅ Kitty terminal (SIGUSR1 signal sent)
- ✅ Alacritty terminal (theme file copied to trigger reload - symlinks don't work due to bug #5852)
- ✅ Btop system monitor (config updated)
- ✅ Wallpaper via hyprpaper (automatically set)

**Manual theming:**
- 📝 Vesktop/Discord (generates custom CSS, manual restart needed)
- 📝 Browsers (no standard API - GTK theme only)

### Theme Script Details

**Location**: `~/.script/theme` (symlinked from `~/.config/nixos/scripts/theme-set`)

**What it does**:
1. Updates current theme symlink (`~/.config/theme/current/theme`)
2. Generates waybar CSS from theme colors + static rules (via `generate-waybar-style`)
3. Generates rofi theme dynamically (via `generate-rofi-theme`)
4. Copies Alacritty theme to real file (not symlink - workaround for bug #5852)
5. Sends reload signals to running applications (Ghostty SIGUSR2, Kitty SIGUSR1, etc.)
6. Updates Hyprland border colors (via `hyprctl`)
7. Sets wallpaper via hyprpaper (first image in theme's backgrounds/ directory)

**Important implementation notes**:
- **Waybar style.css is NOT managed by home-manager** - it's dynamically generated by theme script
- **Rofi theme is dynamically generated** - not managed by home-manager
- **Alacritty theme.toml** is created by home-manager activation script, updated by theme script
- Ghostty/Kitty themes are symlinked by home-manager, work fine with live reload
- Browser theming disabled (no standard API available)

### Wallpaper Selector

**Command**: `wallpaper [theme-name]`

**Features**:
- Interactive fzf-based selection from theme's backgrounds/ directory
- **Pixel-perfect preview** in Ghostty/Kitty terminals (Kitty graphics protocol)
- Text-based preview in other terminals (chafa)
- Auto-detects current theme if no argument provided
- Sets wallpaper immediately via hyprpaper

**Usage**:
```bash
wallpaper              # Select from current theme
wallpaper catppuccin   # Select from specific theme
ghostty -e wallpaper   # Force pixel-perfect preview
```

### Theme System Architecture Notes

**Why themes are outside nix store**:
- NixOS rebuilds were too slow with 20+ themes being copied to nix store
- Solution: Only `base` theme deployed via NixOS (always available for fresh installs)
- Other themes synced via `theme-sync` script from external repo
- Themes stored in `~/.config/nixos/theme/themes/` (in git repo) and `~/.config/theme/themes/` (deployed by home-manager)

## Critical NVIDIA Notes

**RTX 4080 + Hyprland + Multi-Monitor Setup**

### Performance Settings

**GPU power management causes stuttering** - GPU idles at 210 MHz, stutters when ramping to ~2500 MHz.

**Solution applied** (via systemd service in `modules/base/nvidia.nix`):
```bash
nvidia-settings -a "[gpu:0]/GPUPowerMizerMode=1"  # Prefer Maximum Performance
```

This keeps GPU clocks at ~2500 MHz (core) and ~11000 MHz (memory) to prevent lag from clock ramping.

### Hyprland Configuration Requirements

**In `config/hypr/hyprland.conf`**:
- `no_hardware_cursors = true` - Software cursors required for multi-monitor
- `vfr = true` - Variable frame rate for lower GPU usage when idle
- `vrr = 0` - VRR disabled (better for multi-monitor with different refresh rates)
- `enable_hyprcursor = false` - Better cursor performance
- Explicit sync is automatic with NVIDIA 555+ drivers (no manual config needed)

### Waybar Performance

**Disable CSS transitions on `#mpris` module** to prevent mouse lag during song changes:
- Global CSS transitions (`transition: all 0.5s;`) cause compositor lag when waybar text updates
- Solution: `transition: none;` on `#mpris`, keep targeted transitions only for workspaces/tray
- Also set `"interval": 2` and `"tooltip": false` in waybar mpris config

This is handled by the `generate-waybar-style` script.
