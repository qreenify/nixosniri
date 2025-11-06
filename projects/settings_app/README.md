# Hyprland Settings App

A modern, intuitive settings application for the Hyprland Wayland compositor, inspired by COSMIC Settings.

## Vision
Create a user-friendly GUI for managing Hyprland and system configurations without manually editing config files, while still respecting power users who prefer text-based configuration.

## Current Status
🚧 **In Design Phase** - See [DESIGN.md](DESIGN.md) for detailed architecture and planning.

## Planned Features
- 🎨 Appearance customization (themes, fonts, cursors)
- 🖥️ Display management (multi-monitor, resolution, scaling)
- ⌨️ Input device configuration
- 🪟 Hyprland-specific settings (gaps, animations, window rules)
- 🔧 Keybinding management with visual editor
- 🚀 Startup application management
- 💾 Configuration backup and restore

## Tech Stack (Proposed)
- **Language:** Rust
- **UI Framework:** GTK4 with Libadwaita
- **Config Format:** TOML/Hyprland native format
- **IPC:** Hyprland socket communication

## Quick Start (Future)
```bash
# Install from AUR (Arch Linux)
yay -S hyprland-settings

# Or build from source
git clone https://github.com/yourusername/hyprland-settings
cd hyprland-settings
cargo build --release
sudo make install
```

## Development Setup
```bash
# Clone the repository
git clone https://github.com/yourusername/hyprland-settings
cd hyprland-settings

# Install dependencies (Arch Linux)
sudo pacman -S rust gtk4 libadwaita

# Build and run
cargo run
```

## Project Structure
```
settings-app/
├── DESIGN.md           # Detailed design document
├── README.md           # This file
├── src/                # Source code
├── assets/             # Icons and images
└── Cargo.toml          # Rust project file
```

## Contributing
This project is in early development. Contributions, ideas, and feedback are welcome!

### How to Contribute
1. Read the [DESIGN.md](DESIGN.md) document
2. Check existing issues and discussions
3. Fork the repository
4. Create a feature branch
5. Submit a pull request

## Inspiration
- [COSMIC Settings](https://github.com/pop-os/cosmic-settings) - Pop!_OS
- GNOME Settings
- KDE System Settings

## License
[To be decided - likely GPL-3.0 or MIT]

## Contact
[Your contact information]

---
*This project aims to make Hyprland more accessible to users who prefer graphical configuration tools while maintaining the flexibility that makes Hyprland great.*