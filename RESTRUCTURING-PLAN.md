# NixOS Config Restructuring Plan

**Goal**: Transform the chaotic current structure into a clean, organized, professional layout.

---

## Current Problems

1. **Documentation chaos**: 12+ .md files scattered in root
2. **No clear organization**: modules/, config/, theme/, scripts/ all at same level
3. **Mixed concerns**: Personal and base configs intertwined
4. **Build artifacts**: nixos-vm.qcow2, result/, backup-themes/ in repo
5. **Unclear naming**: test-theme.nu, vm-test.sh in root
6. **Hard to navigate**: No clear entry point for newcomers

---

## Proposed Clean Structure

**Note**: modules/ and home/ are MERGED - single-user setup doesn't need the separation!

```
~/.config/nixos/
├── 📋 docs/                     # ALL documentation (move from root)
│   ├── README.md                 # Main entry (link from root)
│   ├── ARCHITECTURE.md
│   ├── INSTALLATION.md
│   ├── MODULES.md
│   ├── THEMING.md
│   ├── VM-TESTING.md
│   ├── AUDIO-CONTROL.md
│   ├── NEOVIM-GUIDE.md
│   ├── HISTORY.md
│   └── archive/                  # Old/reference docs
│       ├── BUILD-TYPES-EXPLAINED.md
│       ├── SEPARATION-PLAN.md
│       ├── RUST-VOLUME-DAEMON.md
│       └── MODULAR-README.md
│
├── 🏠 hosts/                    # Per-machine configurations
│   ├── qreenify/                 # Main machine
│   │   ├── hardware-configuration.nix
│   │   └── configuration.nix     # Host-specific settings
│   │
│   ├── vm/                       # VM for testing
│   │   └── configuration.nix
│   │
│   └── iso/                      # ISO build
│       └── configuration.nix
│
├── 📦 modules/                  # System + User configs (MERGED!)
│   ├── base/                     # Base system (for ISO)
│   │   # System-level configs
│   │   ├── boot.nix              # Bootloader
│   │   ├── networking.nix        # Network manager
│   │   ├── locale.nix            # Timezone, locale
│   │   ├── desktop.nix           # Hyprland/GDM (system)
│   │   ├── audio.nix             # Pipewire
│   │   ├── nvidia.nix            # Nvidia drivers
│   │   ├── packages.nix          # System packages
│   │   ├── users.nix             # User accounts
│   │   # User-level configs (home-manager)
│   │   ├── shell.nix             # Nushell config
│   │   ├── neovim.nix            # Neovim config
│   │   ├── terminals.nix         # Ghostty, Alacritty, Kitty
│   │   ├── hyprland.nix          # Hyprland user config
│   │   ├── theming.nix           # GTK, Qt, cursors, theme system
│   │   └── default.nix           # Imports all base
│   │
│   ├── personal/                 # Personal additions
│   │   # System-level
│   │   ├── gaming.nix            # Steam, gamemode
│   │   ├── virtualization.nix    # virt-manager, libvirtd
│   │   ├── services.nix          # Sunshine, OpenRGB
│   │   ├── packages.nix          # Personal packages
│   │   # User-level
│   │   ├── git.nix               # Git config
│   │   ├── user-services.nix     # User systemd services
│   │   └── default.nix           # Imports all personal
│   │
│   └── optional/                 # Optional features
│       ├── lanzaboote.nix        # Secure boot
│       └── mounts.nix            # Custom mounts
│
├── 🎨 theme/                    # Theme system (unchanged)
│   ├── bin/                      # Theme management scripts
│   ├── config/                   # Walker, Elephant configs
│   ├── default/                  # Default theme
│   └── themes/                   # All themes
│       ├── base/
│       ├── pulsar-theme/
│       ├── gruvbox/
│       └── ... (20+ themes)
│
├── ⚙️ config/                   # Static configs (unchanged)
│   ├── hypr/
│   ├── niri/
│   ├── waybar/
│   ├── fuzzel/
│   └── applications/
│
├── 🔧 scripts/                  # Utility scripts (unchanged)
│   ├── theme
│   ├── theme-sync
│   ├── theme-wallpaper-select
│   ├── app-volume-*
│   └── ...
│
├── 🔨 tools/                    # Build/deploy tools (new)
│   ├── rebuild.sh               # Move from root
│   ├── deploy.sh                # Move from root
│   ├── vm-test.sh               # Move from root
│   └── iso-build.sh             # New
│
├── flake.nix                     # Main flake (restructured)
├── flake.lock
├── .gitignore                    # Add VM artifacts, etc.
├── LICENSE                       # Add for public release
└── README.md                     # Short, links to docs/README.md
```

---

## Migration Steps

### Phase 1: Create New Structure
```bash
cd ~/.config/nixos

# Create new directories
mkdir -p docs/archive
mkdir -p hosts/{qreenify,vm,iso}
mkdir -p modules/{base,personal,optional}
mkdir -p home/{base,personal}
mkdir -p tools
```

### Phase 2: Move Documentation
```bash
# Move all .md files to docs/
mv AUDIO-CONTROL-RESEARCH.md docs/AUDIO-CONTROL.md
mv BUILD-TYPES-EXPLAINED.md docs/archive/
mv INSTALL.md docs/INSTALLATION.md
mv LICENSE docs/  # If exists
mv MODULAR-README.md docs/archive/
mv NEOVIM-GUIDE.md docs/
mv README.md docs/README.md
mv RUST-VOLUME-DAEMON.md docs/archive/
mv SEPARATION-PLAN.md docs/archive/
mv VIRT-MANAGER.md docs/
mv VM-TESTING.md docs/

# Create new root README (short, links to docs/)
cat > README.md << 'EOF'
# Wonderland NixOS

A modular, themeable NixOS distribution with Hyprland, advanced theming, and clean architecture.

## Quick Links
- **[Full Documentation](docs/README.md)**
- **[Installation Guide](docs/INSTALLATION.md)**
- **[Architecture Overview](docs/ARCHITECTURE.md)**
- **[Theme System](docs/THEMING.md)**

## Quick Start
```bash
# Rebuild system
./tools/rebuild.sh

# Switch theme
theme <theme-name>

# Test in VM
./tools/vm-test.sh
```

See [docs/](docs/) for complete documentation.
EOF
```

### Phase 3: Reorganize Modules
```bash
# Move current modules to new structure
mv modules/boot.nix modules/base/
mv modules/networking.nix modules/base/
mv modules/locale.nix modules/base/
mv modules/nvidia.nix modules/base/
mv modules/desktop.nix modules/base/
mv modules/users.nix modules/base/
mv modules/packages-base.nix modules/base/packages.nix

# Move personal modules
mv modules/packages-personal.nix modules/personal/packages.nix

# Move optional modules
mv modules/mounts.nix modules/optional/
# Lanzaboote integration will be in optional/lanzaboote.nix

# Remove old structure
rm modules/packages.nix  # Now in base/packages.nix + personal/packages.nix
rm modules/packages-personal-backup.nix  # Backup file
```

### Phase 4: Reorganize Home-Manager
```bash
# Split home.nix into base and personal
# (This requires actual code changes - see detailed plan below)
```

### Phase 5: Organize Build Tools
```bash
# Move build scripts
mv rebuild.sh tools/
mv deploy.sh tools/
mv vm-test.sh tools/
mv test-theme.nu tools/  # If still needed

# Update paths in scripts (they now run from tools/)
```

### Phase 6: Clean Up Root
```bash
# Add to .gitignore
echo "nixos-vm.qcow2" >> .gitignore
echo "result" >> .gitignore
echo "result-*" >> .gitignore
echo "backup-themes/" >> .gitignore

# Remove generated files from repo
git rm --cached nixos-vm.qcow2
git rm --cached -r backup-themes/
```

### Phase 7: Update Flake
```nix
# flake.nix needs to be updated to use new structure
# See detailed plan below
```

---

## Detailed Code Changes

### 1. New `flake.nix` Structure

```nix
{
  description = "Wonderland NixOS - A modular, themeable distribution";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    lanzaboote.url = "github:nix-community/lanzaboote/v0.4.2";
    home-manager.url = "github:nix-community/home-manager";
    zen-browser.url = "github:0xc000022070/zen-browser-flake";
  };

  outputs = { self, nixpkgs, lanzaboote, home-manager, zen-browser, ... }: {

    # Main system configuration (base + personal)
    nixosConfigurations.wondernixlandos = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      specialArgs = { inherit zen-browser; };
      modules = [
        ./hosts/qreenify/hardware-configuration.nix
        ./hosts/qreenify/configuration.nix

        # Base modules
        ./modules/base

        # Personal modules
        ./modules/personal

        # Optional modules
        ./modules/optional/lanzaboote.nix
        ./modules/optional/mounts.nix

        # Lanzaboote
        lanzaboote.nixosModules.lanzaboote

        # Home Manager
        home-manager.nixosModules.home-manager
        {
          home-manager.useGlobalPkgs = true;
          home-manager.useUserPackages = true;
          home-manager.users.qreenify = import ./hosts/qreenify/home.nix;
        }
      ];
    };

    # VM configuration (base only - for testing)
    nixosConfigurations.wondernixlandos-vm = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      specialArgs = { inherit zen-browser; };
      modules = [
        ./hosts/vm/configuration.nix

        # Base modules only
        ./modules/base

        # Home Manager (base only)
        home-manager.nixosModules.home-manager
        {
          home-manager.useGlobalPkgs = true;
          home-manager.useUserPackages = true;
          home-manager.users.qreenify = import ./home/base;
        }
      ];
    };

    # ISO configuration (base only - for distribution)
    nixosConfigurations.wondernixlandos-iso = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      specialArgs = { inherit zen-browser; };
      modules = [
        ./hosts/iso/configuration.nix

        # Base modules only
        ./modules/base

        # Home Manager (base only)
        home-manager.nixosModules.home-manager
        {
          home-manager.useGlobalPkgs = true;
          home-manager.useUserPackages = true;
          home-manager.users.qreenify = import ./home/base;
        }
      ];
    };
  };
}
```

### 2. `modules/base/default.nix`

```nix
{ ... }:

{
  imports = [
    ./boot.nix
    ./networking.nix
    ./locale.nix
    ./desktop.nix
    ./audio.nix
    ./nvidia.nix
    ./packages.nix
    ./users.nix
  ];
}
```

### 3. `modules/personal/default.nix`

```nix
{ ... }:

{
  imports = [
    ./gaming.nix
    ./virtualization.nix
    ./services.nix
    ./packages.nix
  ];
}
```

### 4. `home/base/default.nix`

```nix
{ ... }:

{
  imports = [
    ./shell.nix
    ./neovim.nix
    ./terminals.nix
    ./desktop.nix
    ./theming.nix
    ./packages.nix
  ];
}
```

### 5. `hosts/qreenify/configuration.nix`

```nix
{ config, pkgs, ... }:

{
  # Host-specific settings
  networking.hostName = "wondernixlandos";

  # Import base modules (handled in flake.nix)
  # Import personal modules (handled in flake.nix)

  system.stateVersion = "25.05";
}
```

### 6. `hosts/qreenify/home.nix`

```nix
{ ... }:

{
  imports = [
    ../../home/base
    ../../home/personal
  ];

  home.username = "qreenify";
  home.homeDirectory = "/home/qreenify";
  home.stateVersion = "25.05";
}
```

### 7. `hosts/vm/configuration.nix`

```nix
{ config, pkgs, lib, modulesPath, ... }:

{
  imports = [
    "${modulesPath}/virtualisation/qemu-vm.nix"
  ];

  # VM-specific settings
  virtualisation.vmVariant = {
    virtualisation = {
      memorySize = 4096;
      cores = 4;
      diskSize = 20000;
      graphics = true;
      qemu.options = [ "-vga virtio" "-display gtk,gl=on" ];
    };
  };

  # Use systemd-boot instead of lanzaboote
  boot.loader.systemd-boot.enable = true;
  boot.loader.efi.canTouchEfiVariables = true;

  networking.hostName = lib.mkForce "wondernixlandos-vm";

  # User account for VM
  users.users.qreenify = {
    isNormalUser = true;
    extraGroups = [ "networkmanager" "wheel" "video" "audio" ];
    password = "test";
  };

  security.sudo.wheelNeedsPassword = false;

  # GDM with auto-login
  services.displayManager.ly.enable = lib.mkForce false;
  services.displayManager.gdm.enable = true;
  services.displayManager.autoLogin = {
    enable = true;
    user = "qreenify";
  };
  services.displayManager.defaultSession = "hyprland";

  nixpkgs.config.allowUnfree = true;
  system.stateVersion = "25.05";
}
```

---

## Benefits of New Structure

### For You (qreenify)
- ✅ **Clear organization**: Know where everything is
- ✅ **Easy navigation**: Logical hierarchy
- ✅ **Better git history**: Organized commits
- ✅ **Faster development**: Find files quickly
- ✅ **Clean root**: No clutter

### For Other Claudes
- ✅ **Clear entry point**: docs/README.md
- ✅ **Context awareness**: NIXOS-PROJECT-OVERVIEW.md
- ✅ **Easy orientation**: Logical structure
- ✅ **No confusion**: Everything in its place

### For Public Release
- ✅ **Professional**: Industry-standard layout
- ✅ **Approachable**: Easy for newcomers
- ✅ **Documented**: Clear guides
- ✅ **Modular**: Easy to fork and customize

---

## Execution Plan

### Timeline
1. **Phase 1-2**: Create structure, move docs (30 min)
2. **Phase 3-5**: Reorganize modules, home, tools (1 hour)
3. **Phase 6**: Clean up root (15 min)
4. **Phase 7**: Update flake.nix (45 min)
5. **Testing**: Build main system, VM, test (30 min)
6. **Documentation**: Update all docs with new paths (1 hour)

**Total estimated time**: ~4 hours

### Rollback Plan
- Keep backup branch before restructuring
- Test VM build before touching main system
- Commit each phase separately
- Can revert individual phases if needed

---

## Next Steps

1. **Get approval** on this structure
2. **Create backup branch**: `git checkout -b pre-restructure-backup`
3. **Execute phases** one by one
4. **Test thoroughly** after each phase
5. **Update documentation** with new structure
6. **Commit with clear messages**

---

**Ready to execute?** This will make your config professional, clean, and ready for public release! 🚀
