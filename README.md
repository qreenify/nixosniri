# NixOS Configuration - Simple & Modular

Clean, flat module structure. Each file does one thing.

## Structure

```
.
├── flake.nix                      # Main entry point
├── hardware-configuration.nix     # Auto-generated hardware config
│
├── modules/
│   ├── boot.nix                   # Bootloader, kernel, secure boot
│   ├── networking.nix             # Network configuration
│   ├── locale.nix                 # Time zone, keyboard, locales
│   ├── nvidia.nix                 # GPU drivers
│   ├── packages.nix               # ALL system packages
│   ├── users.nix                  # User accounts
│   ├── desktop.nix                # Niri, waybar, desktop environment
│   └── home.nix                   # Home-manager (themes, configs)
│
├── config/                        # Actual config files
│   ├── niri/config.kdl            # Niri compositor
│   ├── waybar/                    # Status bar
│   └── fuzzel/                    # Launcher
│
└── scripts/                       # Helper scripts
    ├── mic-status
    ├── mic-toggle
    └── audio-out-status
```

## What Each Module Does

| File | Contains |
|------|----------|
| `boot.nix` | Bootloader, kernel, Lanzaboote secure boot, Nix settings |
| `networking.nix` | Hostname, NetworkManager |
| `locale.nix` | Time zone, keyboard layout (Swedish), locales |
| `nvidia.nix` | NVIDIA GPU drivers and settings |
| `packages.nix` | **ALL system packages** - add packages here |
| `users.nix` | User accounts and groups |
| `desktop.nix` | Niri, waybar, display manager, security |
| `home.nix` | User configs: niri, waybar, alacritty, dark theme |

## Installation

```bash
# 1. Backup
sudo cp -r /etc/nixos /etc/nixos.backup

# 2. Deploy configs (automated)
cd /home/qreenify/claude
./deploy.sh

# 3. Install scripts
mkdir -p ~/.script
cp scripts/* ~/.script/
chmod +x ~/.script/*

# 4. Rebuild
cd /etc/nixos
sudo rm flake.lock
sudo nix flake update
sudo nixos-rebuild switch --flake /etc/nixos#nixos
```

## Daily Workflow

After the initial setup, use these aliases for quick updates:

```bash
# Edit your config in ~/claude, then:
deploy   # Copy changes to /etc/nixos
rebuild  # Apply changes (rebuild NixOS)

# Other useful aliases:
n        # Open nvim
```

## 📚 Learning Neovim

Check out [NEOVIM-GUIDE.md](NEOVIM-GUIDE.md) for:
- Complete keybinding reference
- Learning path from beginner to advanced
- Tips and tricks to improve your workflow
- **Pro tip:** Press `Space` in nvim and wait - which-key will show you available commands!

## How to Modify

### Add a package
Edit `modules/packages.nix`:
```nix
environment.systemPackages = with pkgs; [
  # ... existing packages
  firefox
];
```

### Change keyboard layout
Edit `modules/locale.nix`:
```nix
services.xserver.xkb = {
  layout = "us";  # Change from "se"
  variant = "";
};
```

### Change network hostname
Edit `modules/networking.nix`:
```nix
networking.hostName = "my-new-hostname";
```

### Add another user
Edit `modules/users.nix`:
```nix
users.users.newuser = {
  isNormalUser = true;
  extraGroups = [ "wheel" ];
};
```

### Change niri keybindings
Edit `config/niri/config.kdl` directly

### Customize colors
- Alacritty: `modules/home.nix` (search for `colors`)
- Waybar: `config/waybar/style.css`
- Niri borders: `config/niri/config.kdl` (search for `border`)

## Monitor Setup

Your 4-monitor setup is in `config/niri/config.kdl`:
- **HDMI-A-1**: 1920x1080@60Hz (top)
- **DP-1**: 2560x1440@60Hz rotated 90° (right)
- **DP-2**: 2560x1440@155Hz (main)
- **DP-3**: 2560x1440@60Hz rotated 270° (left)

## Window Auto-Placement

See `config/niri/config.kdl` for window rules:
- Discord → Workspace 1, DP-1
- Brave PWAs → Workspace 1, DP-3
- VSCodium → Workspace 2, DP-2 (maximized)
- Gaming (Steam, Lutris) → Workspace 3
- OBS → Workspace 4

## Key Features

✅ **Simple** - Flat structure, obvious names
✅ **Modular** - Each file does one thing
✅ **Dark mode** - GTK, Qt, Alacritty, Waybar
✅ **4-monitor setup** - Pre-configured with rotation
✅ **Secure boot** - Lanzaboote v0.4.2 (working)
✅ **Home Manager** - Declarative user configs

## Troubleshooting

### Build fails
```bash
cd /etc/nixos
nix flake check  # Validate syntax
```

### Want to disable NVIDIA?
Comment out in `flake.nix`:
```nix
# ./modules/nvidia.nix
```

### Want to test without rebuilding?
```bash
sudo nixos-rebuild test --flake /etc/nixos#nixos
```

## What Changed From Old Config

Your old `configuration.nix` is now split into 8 clear modules. Everything is the same, just organized better.

- Want to add packages? → `modules/packages.nix`
- Want to change boot? → `modules/boot.nix`
- Want to modify theme? → `modules/home.nix`
- Want to add user? → `modules/users.nix`

No more hunting through a giant file!
