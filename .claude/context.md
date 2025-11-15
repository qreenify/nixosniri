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
- **Home-manager file conflicts**: ALWAYS use `force = true` when deploying files that might already exist
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

- `~/.config/nixos/.git` - NixOS configuration repo
- `~/claude/.git` - Projects and development repo

These are **separate repositories** - don't confuse them.

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
