# Project Context for Claude Code

## Directory Structure

This workspace (`~/claude/`) is for **projects and general development work**.

**NixOS configuration files are NOT here** - they live in `~/.config/nixos/`

### Structure Overview

```
~/.config/nixos/          # NixOS system configuration (separate git repo)
├── flake.nix
├── modules/              # System modules (boot, networking, packages, etc.)
├── config/               # Application configs (hypr, waybar, rofi, ghostty, etc.)
├── scripts/              # System scripts (theme, audio, mic controls)
├── tools/                # Build tools (rebuild.sh, deploy.sh, vm-test.sh)
└── theme/                # Theme system (themes, configs, scripts)

~/claude/                 # This directory - projects & development
├── .claude/              # Claude Code context (this file)
└── projects/             # Development projects
```

## System Information

### Shell & Environment
- **Shell**: Fish (supports both `;` and `&&` for command chaining)
- **GPU**: NVIDIA GeForce RTX 4080
- **Monitors**: 4 displays (DP-2 main 2560x1440@155Hz, others at 60Hz)

### Desktop Environment
- **Compositor**: Hyprland (primary), Niri (available)
- **Bar**: Waybar (started by Hyprland, NOT systemd)
- **Launcher**: Rofi (with plugins: rofi-calc, rofi-file-browser, rofimoji)
- **Terminal**: Ghostty (primary)
- **Browser**: Zen (primary), Brave (installed)

### Critical NixOS Behavior
- **Config changes require rebuild**: `rebuild` command
- **Changes take effect**: After rebuild AND logout/login (or reboot for kernel/boot changes)
- **Waybar config changes**: Require rebuild, then waybar reload

### ⚠️ CRITICAL: Home-manager File Conflicts
**ALWAYS use `force = true` for ALL xdg.configFile and home.file declarations!**

```nix
# For files in ~/.config/
xdg.configFile."somedir" = {
  source = ../path;
  recursive = true;
  force = true;  # Prevents "file would be clobbered" errors
};

# For files elsewhere in $HOME
home.file.".local/share/somedir" = {
  source = ../path;
  recursive = true;
  force = true;
};
```

## Important Rules

### When Working with NixOS Configs

**ALWAYS edit files in `~/.config/nixos/`** when the user wants to:
- Add/remove system packages
- Modify system settings
- Change boot configuration
- Update desktop environment settings
- Modify hypr/waybar/rofi/niri configs
- Update any NixOS module

**NEVER** edit NixOS config files in `~/claude/` - they don't exist here.

**ALWAYS remind user to rebuild after config changes**: `rebuild`

### When Working in ~/claude

This directory is for:
- Development projects (in `projects/`)
- Non-NixOS configuration
- General development work

## Key Commands

- `rebuild` - Deploy and rebuild NixOS
- `rebuild dry-build` - Preview what would change
- `rebuild test` - Test config without boot entry
- `theme <name>` - Switch theme
- `wallpaper` - Interactive wallpaper selector with preview

## Git Repositories

- `~/.config/nixos/.git` - NixOS configuration (synced to GitHub, auto-commit)
- `~/claude/.git` - Projects and development (auto-backup every hour)

These are **separate repositories** - don't confuse them.

## Dual Boot Setup

Windows bootloader lives on separate EFI partition (`/dev/nvme2n1p3`). To make it appear in systemd-boot:

```bash
sudo mkdir -p /mnt/win-efi
sudo mount /dev/nvme2n1p3 /mnt/win-efi
sudo mkdir -p /boot/EFI/Microsoft
sudo cp -r /mnt/win-efi/EFI/Microsoft/Boot /boot/EFI/Microsoft/
sudo umount /mnt/win-efi
```

This persists across rebuilds (NixOS doesn't touch `/boot/EFI/Microsoft/`).

## Wonderland Theme System

### Overview
- **Theme location**: `~/.config/nixos/theme/themes/` and `~/.config/theme/themes/`
- **Current theme symlink**: `~/.config/theme/current/theme`
- **Commands**: `theme <name>` to switch, `wallpaper` for interactive selector

### Supported Applications (Live Reload)
- Waybar (CSS generated dynamically)
- Hyprland (border colors)
- Mako notifications
- Rofi (theme generated dynamically)
- Ghostty (copies theme file, sends reload signal)
- Btop, wallpaper via hyprpaper

### Implementation Notes
- Waybar/Rofi themes are dynamically generated (NOT managed by home-manager)
- Ghostty theme copied (not symlinked) to trigger reload on change
- Wallpaper selector uses fzf with pixel-perfect preview in Ghostty (Kitty protocol)

## Critical NVIDIA Setup (RTX 4080)

**Performance issue**: GPU power management causes stuttering when clock speeds ramp up/down.

**Solution**: Force maximum performance mode
```bash
nvidia-settings -a "[gpu:0]/GPUPowerMizerMode=1"
```

**Hyprland config requirements**:
- `no_hardware_cursors = true` (required for multi-monitor)
- `vfr = true` (variable frame rate)
- `vrr = 0` (VRR disabled for mixed refresh rates)
- Explicit sync automatic with NVIDIA 555+ drivers

**Waybar performance**: Disable CSS transitions on `#mpris` module to prevent lag during song changes.
