# Modular NixOS Configuration

This is a clean, modular NixOS configuration with flakes, home-manager, and niri compositor.

## Directory Structure

```
/etc/nixos/
├── flake.nix                          # Main flake entry point
├── hardware-configuration.nix         # Auto-generated hardware config
│
├── modules/
│   ├── system/                        # System-level configuration
│   │   ├── boot.nix                   # Bootloader, kernel, Lanzaboote
│   │   ├── networking.nix             # Network settings
│   │   └── locale.nix                 # Time zone, locale, keyboard
│   │
│   ├── hardware/                      # Hardware-specific modules
│   │   └── nvidia.nix                 # NVIDIA GPU configuration
│   │
│   ├── users/                         # User configurations
│   │   ├── qreenify.nix               # System user settings
│   │   └── home.nix                   # Home-manager entry point
│   │       └── home/                  # Home-manager modules
│   │           ├── desktop/           # Desktop environment configs
│   │           │   ├── niri.nix       # Niri compositor
│   │           │   ├── waybar.nix     # Status bar
│   │           │   └── fuzzel.nix     # Application launcher
│   │           ├── programs/          # Application configs
│   │           │   └── alacritty.nix  # Terminal emulator
│   │           └── theme/             # Theming configs
│   │               └── dark-mode.nix  # Dark theme (GTK/Qt)
│   │
│   └── desktop/                       # Desktop environment system config
│       └── niri.nix                   # Niri system settings
│
├── config/                            # Configuration files
│   ├── niri/
│   │   └── config.kdl                 # Niri compositor config
│   ├── waybar/
│   │   ├── config.jsonc               # Waybar modules
│   │   └── style.css                  # Waybar styling
│   └── fuzzel/
│       └── fuzzel.ini                 # Fuzzel launcher config
│
└── scripts/                           # Helper scripts
    ├── mic-status                     # Waybar mic status
    ├── mic-toggle                     # Waybar mic toggle
    └── audio-out-status               # Waybar audio output
```

## Module Organization

### System Modules (`modules/system/`)

**boot.nix** - Boot and system basics
- Lanzaboote secure boot
- Kernel selection
- Nix flakes
- Unfree packages
- State version

**networking.nix** - Network configuration
- Hostname
- NetworkManager

**locale.nix** - Localization
- Time zone
- Locale settings
- Keyboard layout (Swedish)

### Hardware Modules (`modules/hardware/`)

**nvidia.nix** - NVIDIA GPU configuration
- Graphics drivers
- NVIDIA-specific settings

### User Modules (`modules/users/`)

**qreenify.nix** - System user configuration
- User account settings
- Groups
- System-wide user packages

**home.nix** - Home-manager entry point
- Imports all home modules
- Common packages
- XDG configuration

### Home-Manager Modules (`modules/users/home/`)

**desktop/** - Desktop environment configurations
- `niri.nix` - Niri compositor config file link
- `waybar.nix` - Waybar status bar
- `fuzzel.nix` - Application launcher

**programs/** - Application configurations
- `alacritty.nix` - Terminal with dark color scheme

**theme/** - Theming
- `dark-mode.nix` - GTK/Qt dark theme settings

### Desktop Module (`modules/desktop/`)

**niri.nix** - System-level niri settings
- Enable niri
- Waybar
- Display manager (ly)
- Desktop packages

## Configuration Files (`config/`)

These are the actual configuration files for desktop applications:

- **niri/config.kdl** - Complete niri compositor config (4 monitors, keybinds, window rules)
- **waybar/** - Status bar configuration and styling
- **fuzzel/fuzzel.ini** - Application launcher settings

## Benefits of This Structure

### ✅ Easy to Understand
Each file has a single, clear purpose. Want to change network settings? Edit `modules/system/networking.nix`.

### ✅ Easy to Maintain
- Small, focused files
- Clear separation of concerns
- Easy to find what you need

### ✅ Easy to Extend
Add new modules by:
1. Create a new `.nix` file in the appropriate module directory
2. Add it to `flake.nix` imports
3. Rebuild

### ✅ Easy to Share
Share specific configurations:
- Just the niri setup? Share `modules/desktop/` and `modules/users/home/desktop/niri.nix`
- Just the theme? Share `modules/users/home/theme/dark-mode.nix`

### ✅ Easy to Version Control
Small files = clear git diffs. Easy to see what changed.

## How to Add a New Module

### Example: Adding Firefox configuration

1. **Create the module file:**
```bash
# For system-wide Firefox
echo '{ pkgs, ... }: { programs.firefox.enable = true; }' > modules/desktop/firefox.nix

# OR for user-level Firefox (home-manager)
mkdir -p modules/users/home/programs
vim modules/users/home/programs/firefox.nix
```

2. **Add to flake.nix or home.nix:**
```nix
# In flake.nix for system-wide:
./modules/desktop/firefox.nix

# In modules/users/home.nix for user-level:
imports = [
  ./home/programs/firefox.nix
  # ...
];
```

3. **Rebuild:**
```bash
sudo nixos-rebuild switch --flake /etc/nixos#nixos
```

## How to Modify Existing Config

### Change keyboard layout
Edit `modules/system/locale.nix`:
```nix
services.xserver.xkb = {
  layout = "us";  # Change from "se"
  variant = "";
};
```

### Add packages to user
Edit `modules/users/qreenify.nix`:
```nix
packages = with pkgs; [
  # ... existing packages
  firefox
  thunderbird
];
```

### Change niri keybindings
Edit `config/niri/config.kdl` directly

### Adjust dark theme colors
Edit `modules/users/home/programs/alacritty.nix` for terminal colors
Edit `config/waybar/style.css` for waybar colors

## Installation

```bash
# 1. Backup current config
sudo cp -r /etc/nixos /etc/nixos.backup

# 2. Copy new structure
cd /home/qreenify/claude
sudo cp flake.nix /etc/nixos/
sudo cp -r modules /etc/nixos/
sudo cp -r config /etc/nixos/

# 3. Install scripts
mkdir -p ~/.script
cp scripts/* ~/.script/
chmod +x ~/.script/*

# 4. Update and rebuild
cd /etc/nixos
sudo rm flake.lock  # Clean start
sudo nix flake update
sudo nixos-rebuild switch --flake /etc/nixos#nixos
```

## Troubleshooting

### Module not found
Make sure the path in `flake.nix` or import statement matches the actual file location.

### Syntax errors
Run `nix flake check` to validate your configuration before rebuilding.

### Want to disable a module temporarily?
Comment it out in `flake.nix`:
```nix
# ./modules/hardware/nvidia.nix  # Disabled
```

## Migration from Monolithic Config

Your old `configuration.nix` has been split into:

| Old Section | New Location |
|------------|--------------|
| Boot & kernel | `modules/system/boot.nix` |
| Networking | `modules/system/networking.nix` |
| Locale & keyboard | `modules/system/locale.nix` |
| NVIDIA | `modules/hardware/nvidia.nix` |
| User account | `modules/users/qreenify.nix` |
| Niri & desktop | `modules/desktop/niri.nix` |
| User packages & themes | `modules/users/home.nix` and subdirectories |

## Further Customization Ideas

### Create profile modules
```
modules/profiles/
  ├── gaming.nix       # Steam, Lutris, gaming packages
  ├── development.nix  # Dev tools, IDEs
  └── media.nix        # OBS, video editing
```

### Per-application configs
```
modules/users/home/programs/
  ├── git.nix
  ├── neovim.nix
  ├── firefox.nix
  └── vscode.nix
```

### Environment-specific configs
```
modules/environments/
  ├── work.nix
  ├── personal.nix
  └── gaming.nix
```

Then in `flake.nix`, conditionally import based on hostname or variable.

## Resources

- [NixOS Module System](https://nixos.org/manual/nixos/stable/#sec-writing-modules)
- [Home Manager Modules](https://nix-community.github.io/home-manager/index.xhtml#ch-writing-modules)
- [Nix Flakes](https://nixos.wiki/wiki/Flakes)
