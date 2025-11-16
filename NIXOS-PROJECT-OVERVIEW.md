# Wonderland NixOS - Project Overview

> **Central documentation for all Claude instances working on this project**

Last Updated: 2025-11-16

---

## 🎯 Project Mission

Building a **modular, themeable NixOS distribution** with:
- Clean base system suitable for ISO distribution
- Personal overlay for qreenify's gaming/productivity setup
- Advanced theming system with 20+ themes
- Professional documentation and structure

---

## 🏗️ Technology Stack

### Core System
- **OS**: NixOS (unstable channel)
- **Configuration**: Flakes-based declarative config
- **User Management**: home-manager (integrated)
- **Boot**: Lanzaboote (Secure Boot support)

### Desktop Environment
- **Compositor**: Hyprland (primary) + Niri (alternative)
- **Display Manager**: GDM (with auto-login support)
- **Shell**: Nushell (modern, structured shell)
- **Terminal**: Ghostty (base), Kitty + Alacritty (extras)
- **Launcher**: Walker (primary), Fuzzel (fallback)
- **Bar**: Waybar (custom modules for app audio control)
- **Notifications**: Mako
- **Lock Screen**: Swaylock + Hyprlock

### Development Tools
- **Editor**: Neovim (fully configured with LSP, completion, etc.)
- **Version Control**: Git + GitHub CLI
- **AI Assistant**: Claude Code

### Theme System
- **Engine**: Custom bash/nushell scripts
- **Themes**: 20+ themes (pulsar, catppuccin, gruvbox, dracula, etc.)
- **Supported Apps**:
  - Terminals: Alacritty, Kitty, Ghostty
  - Browser: Chromium/Brave styling
  - Desktop: Waybar, Hyprland, Mako, Walker
  - CLI: Neovim, btop
- **Live Switching**: Change themes without logout
- **Wallpaper System**: Per-theme backgrounds with preview

---

## 📂 Repository Structure (Current vs. Planned)

### Current Structure (MESSY - TO BE REORGANIZED)
```
~/.config/nixos/
├── Many .md files scattered in root
├── modules/ - system modules
├── config/ - desktop configs
├── theme/ - theme system
├── scripts/ - utility scripts
├── flake.nix
└── (chaos)
```

### Planned Structure (CLEAN)
```
~/.config/nixos/
├── 📋 docs/                     # All documentation here
│   ├── README.md                 # Main entry point
│   ├── ARCHITECTURE.md           # System design
│   ├── INSTALLATION.md           # Install guide
│   ├── MODULES.md                # Module documentation
│   ├── THEMING.md                # Theme system guide
│   ├── VM-TESTING.md             # VM setup and testing
│   ├── AUDIO-CONTROL.md          # Per-app audio control
│   └── HISTORY.md                # What we've built together
│
├── 🏠 hosts/                    # Per-machine configurations
│   ├── qreenify/                 # Personal machine (base + personal)
│   │   ├── hardware-configuration.nix
│   │   ├── host.nix              # Machine-specific settings
│   │   └── home.nix              # User config (imports base + personal)
│   │
│   ├── vm/                       # VM configuration (base only)
│   │   └── vm.nix
│   │
│   └── iso/                      # ISO configuration (base only)
│       └── iso.nix
│
├── 📦 modules/                  # Reusable NixOS modules
│   ├── base/                     # Base system (for ISO)
│   │   ├── boot.nix
│   │   ├── networking.nix
│   │   ├── locale.nix
│   │   ├── desktop.nix
│   │   ├── audio.nix
│   │   ├── packages.nix
│   │   └── nvidia.nix (optional)
│   │
│   └── personal/                 # Personal additions
│       ├── gaming.nix
│       ├── virtualization.nix
│       ├── services.nix
│       └── packages.nix
│
├── 👤 home/                     # Home-manager configurations
│   ├── base/                     # Base user config (for ISO)
│   │   ├── shell.nix             # Nushell config
│   │   ├── neovim.nix            # Editor config
│   │   ├── terminals.nix         # Terminal configs
│   │   ├── desktop.nix           # Hyprland, Waybar, etc.
│   │   ├── theming.nix           # GTK, Qt, cursors
│   │   └── default.nix           # Imports all base
│   │
│   └── personal/                 # Personal additions
│       ├── git.nix               # Personal git config
│       ├── services.nix          # Personal systemd services
│       ├── autostarts.nix        # App autostarts
│       └── default.nix           # Imports all personal
│
├── 🎨 theme/                    # Theme system
│   ├── bin/                      # Theme scripts
│   ├── config/                   # Theme configs (walker, elephant, etc.)
│   ├── default/                  # Default theme
│   └── themes/                   # All themes
│       ├── pulsar-theme/
│       ├── gruvbox/
│       ├── catppuccin/
│       └── ... (20+ themes)
│
├── ⚙️ config/                   # Static desktop configs
│   ├── hypr/                     # Hyprland configs
│   ├── niri/                     # Niri configs
│   ├── waybar/                   # Waybar config
│   └── fuzzel/                   # Fuzzel config
│
├── 🔧 scripts/                  # Utility scripts
│   ├── theme                     # Theme switcher
│   ├── theme-sync                # Theme downloader
│   ├── app-volume-*              # Audio control scripts
│   └── ...
│
├── 🔨 tools/                    # Build and deploy tools
│   ├── rebuild.sh                # nixos-rebuild wrapper
│   ├── deploy.sh                 # Deploy to /etc/nixos
│   ├── vm-test.sh                # VM testing
│   └── iso-build.sh              # ISO builder
│
├── flake.nix                     # Main flake
├── flake.lock                    # Locked dependencies
└── .gitignore
```

---

## 🎯 Three-Repository Strategy

### 1. **wonderland-nixos** (BASE - Public)
- **Location**: `~/.config/nixos/` (restructured)
- **Purpose**: Distributable base system
- **Contains**:
  - Clean base modules
  - Theme system (all 20+ themes)
  - ISO build configuration
  - Documentation for users
- **Audience**: Anyone wanting to use Wonderland NixOS

### 2. **wonderland-personal** (OVERLAY - Private)
- **Purpose**: Personal additions on top of base
- **Contains**:
  - Gaming setup (Steam, Lutris, Sunshine, OpenRGB)
  - Virtualization (virt-manager)
  - Extra apps (Discord, Zen browser, Claude Code)
  - Personal git config, secrets
  - Personal systemd services
- **Usage**: Imports wonderland-nixos as flake input
- **Status**: To be extracted later (currently in modules/personal/)

### 3. **wonderland-iso** (ISO BUILDER - Public)
- **Location**: `~/git/wonderland-iso/`
- **Purpose**: Build bootable ISO from base
- **Contains**:
  - ISO-specific configuration
  - Installer scripts
  - Branding (splash screens)
- **Uses**: Only base modules from wonderland-nixos
- **Status**: Created but needs updating

---

## 🚀 What We've Built Together

### 1. **Secure Boot with Lanzaboote**
- TPM-based secure boot
- Automatic key management
- UEFI integration

### 2. **Advanced Theme System**
- 20+ complete themes
- Live switching without logout
- Supports: terminals, browsers, desktop, waybar, neovim
- Wallpaper system with preview
- Per-theme configurations

### 3. **Per-Application Audio Control**
- Custom Waybar module
- Tray icons for each app
- PipeWire integration
- Real-time volume monitoring
- Rust daemon for performance

### 4. **Modular Configuration**
- Base/personal separation
- Optional modules (gaming, virtualization, nvidia)
- Reusable across machines
- Clean VM and ISO builds

### 5. **VM Testing Environment**
- Base-only VM (fast, clean)
- GDM with Hyprland
- No personal bloat
- Quick testing of changes

### 6. **Custom Scripts & Automation**
- Theme management scripts
- Auto-backup to GitHub
- Cloud storage integration (rclone)
- Deployment automation

---

## 🎨 Theme System Details

### How It Works
1. **Theme Storage**: `~/.config/theme/themes/`
2. **Active Theme**: Symlink at `~/.config/theme/current/theme`
3. **Switching**: `theme <theme-name>` command
4. **Components**: Each theme includes:
   - Alacritty, Kitty, Ghostty configs
   - Hyprland colors
   - Waybar styling
   - Mako colors
   - Walker CSS
   - Neovim theme
   - Chromium/Brave theme
   - Wallpaper(s)

### Available Themes (Base Set)
1. pulsar-theme
2. matte-black
3. gruvbox
4. dracula
5. tokyo-night
6. catppuccin
7. catppuccin-light
8. everforest

*(Plus 12+ more in full collection)*

---

## 🔧 Key Commands

### System Management
```bash
rebuild          # Rebuild NixOS (wrapper for nixos-rebuild)
deploy           # Copy config to /etc/nixos and rebuild
```

### Theme Management
```bash
theme <name>     # Switch theme
wallpaper        # Select wallpaper interactively
theme-sync       # Download/update themes from cloud
```

### Cloud Storage
```bash
gdrive-start     # Mount Google Drive
gdrive-stop      # Unmount Google Drive
gdrive-status    # Check mount status
```

### Development
```bash
n <file>         # Open in neovim (alias)
```

---

## 📋 Current State (Nov 2025-11-16)

### ✅ Completed
- [x] Base/personal separation implemented
- [x] VM configuration (base-only, fast)
- [x] Theme system fully functional
- [x] Secure boot with Lanzaboote
- [x] Per-app audio control
- [x] Home-manager integration
- [x] Neovim setup with LSP
- [x] Hyprland + Niri compositors
- [x] Documentation started

### 🚧 In Progress
- [ ] Restructure directories (docs/, hosts/, modules/base, modules/personal, etc.)
- [ ] Consolidate documentation files
- [ ] Update wonderland-iso repo with clean base
- [ ] Extract personal config to separate repo (optional)

### 📝 Planned
- [ ] Add remaining themes to base (12 more)
- [ ] Create installation ISO and test
- [ ] Public release preparation
- [ ] User documentation
- [ ] Example configurations

---

## 🎯 Design Principles

1. **Modularity**: Everything is a module that can be enabled/disabled
2. **Declarative**: All configuration in Nix files, no imperative setup
3. **Reproducible**: Same config = same system, always
4. **Themeable**: Complete visual customization without rebuilding
5. **Fast**: Optimized for quick rebuilds and theme switching
6. **Clean**: Well-organized, documented, understandable
7. **Shareable**: Base system suitable for distribution

---

## 🤝 For Future Claude Instances

When you start working on this project:

1. **Read this file first** - It's your orientation guide
2. **Check `docs/ARCHITECTURE.md`** - Understanding the system design
3. **Review recent commits** - See what's changed
4. **Ask questions** - User knows this system intimately

### Common Tasks
- **Adding a theme**: Add to `theme/themes/`, ensure all components present
- **Modifying base**: Edit files in `modules/base/` and `home/base/`
- **Personal changes**: Edit files in `modules/personal/` and `home/personal/`
- **Testing**: Use VM (`vm-test.sh`) before deploying to main system
- **Documentation**: Update relevant files in `docs/`

### Important Context
- User is qreenify
- Primary machine: wondernixlandos (gaming + productivity)
- Uses Hyprland on Wayland
- Nvidia GPU (requires special handling)
- Prefers declarative over imperative
- Values clean code and good documentation
- Building this for both personal use AND public distribution

---

## 📞 Getting Help

- **NixOS Manual**: https://nixos.org/manual/nixos/stable/
- **Home Manager**: https://nix-community.github.io/home-manager/
- **Hyprland Wiki**: https://wiki.hyprland.org/
- **Nushell Book**: https://www.nushell.sh/book/

---

*This document is the source of truth for the Wonderland NixOS project. Keep it updated!*
