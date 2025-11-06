# Hyprland Settings App Design Document

## Overview
A modern, user-friendly settings application for Hyprland window manager, inspired by COSMIC Settings design and functionality.

## Goals
- Provide intuitive GUI for configuring Hyprland and system settings
- Match the clean, modern aesthetic of COSMIC Settings
- Seamless integration with Hyprland configuration files
- Real-time preview of changes where possible
- Support for both light and dark themes

## Tech Stack Considerations

### Option 1: Rust + GTK4/Libadwaita (Recommended)
**Pros:**
- Same stack as COSMIC Settings
- Modern, performant, memory-safe
- Excellent Wayland support
- Beautiful native widgets with Libadwaita
- Good theming support

**Cons:**
- Steeper learning curve if new to Rust
- GTK4 documentation can be sparse

### Option 2: Rust + Iced
**Pros:**
- Pure Rust solution
- Custom, modern UI design
- Great for custom widgets
- Lighter than GTK

**Cons:**
- Less mature ecosystem
- More work for standard widgets

### Option 3: Flutter
**Pros:**
- Beautiful UI out of the box
- Fast development
- Cross-platform if needed
- Hot reload for development

**Cons:**
- Larger bundle size
- Less "native" feel
- Dart language overhead

## Core Features

### 1. System Settings
- **Appearance**
  - Theme selection (Light/Dark/Auto)
  - Icon themes
  - Cursor themes
  - Font configuration
  - Window decorations

- **Display**
  - Monitor configuration
  - Resolution settings
  - Refresh rate
  - Scaling
  - Multi-monitor setup

- **Input Devices**
  - Keyboard layout
  - Key repeat rate
  - Mouse sensitivity
  - Touchpad gestures
  - Scroll behavior

### 2. Hyprland-Specific Settings
- **Window Management**
  - Gaps configuration
  - Border settings
  - Window rules
  - Workspace behavior
  - Animation settings

- **Keybindings**
  - View current bindings
  - Add/modify/remove bindings
  - Import/export binding schemes

- **Performance**
  - VFR (Variable Frame Rate) settings
  - Damage tracking
  - Backend options
  - GPU settings

### 3. Application Settings
- **Default Applications**
  - Terminal emulator
  - File manager
  - Web browser
  - Text editor

- **Startup Applications**
  - Manage autostart programs
  - Enable/disable entries
  - Add custom commands

### 4. User & System
- **User Account**
  - Avatar management
  - User information
  - Password change

- **About**
  - System information
  - Hyprland version
  - Hardware details

## Architecture

### Frontend Structure
```
settings-app/
├── src/
│   ├── main.rs              # Application entry point
│   ├── app.rs               # Main application logic
│   ├── config/              # Configuration management
│   │   ├── mod.rs
│   │   ├── hyprland.rs      # Hyprland config parser/writer
│   │   └── system.rs        # System config handlers
│   ├── ui/                  # UI components
│   │   ├── mod.rs
│   │   ├── sidebar.rs       # Navigation sidebar
│   │   ├── header.rs        # Header bar
│   │   └── pages/           # Individual settings pages
│   │       ├── appearance.rs
│   │       ├── display.rs
│   │       ├── input.rs
│   │       └── ...
│   └── utils/               # Utility functions
│       ├── mod.rs
│       └── ipc.rs          # Hyprland IPC communication
├── assets/                  # Icons, images
├── Cargo.toml              # Rust dependencies
└── README.md
```

### Backend Design
- **Configuration Management**
  - Read/parse Hyprland config files
  - Write changes atomically
  - Backup before modifications
  - Support for config.d/ directory structure

- **IPC Communication**
  - Use Hyprland's IPC socket for real-time updates
  - Apply settings without restart where possible
  - Monitor for external config changes

- **Data Persistence**
  - Store app preferences in XDG config directory
  - Cache frequently accessed data
  - Handle config migrations

## UI/UX Design Principles

### Layout
- **Two-panel design**
  - Left: Category navigation (icon + text)
  - Right: Settings content area
  - Responsive sizing

### Visual Design
- Clean, modern interface
- Consistent spacing (8px grid)
- Card-based layout for setting groups
- Clear visual hierarchy
- Smooth transitions and animations

### Interaction
- Instant apply where safe
- Apply/Revert for critical changes
- Search functionality
- Keyboard navigation support
- Tooltips for complex options

## Implementation Phases

### Phase 1: Foundation (Week 1-2)
- Set up project structure
- Implement basic UI framework
- Create navigation system
- Design data models

### Phase 2: Core Features (Week 3-4)
- Appearance settings
- Display configuration
- Basic Hyprland config parsing
- Config file writing

### Phase 3: Advanced Features (Week 5-6)
- Keybinding management
- Window rules editor
- IPC integration
- Real-time preview

### Phase 4: Polish (Week 7-8)
- Animations and transitions
- Error handling
- Testing
- Documentation
- Package for distribution

## Testing Strategy
- Unit tests for config parsing/writing
- Integration tests for IPC communication
- Manual testing on various Hyprland setups
- User acceptance testing

## Distribution
- AUR package for Arch Linux
- Flatpak for universal distribution
- Native packages for major distributions
- Build from source instructions

## Future Enhancements
- Plugin system for extensions
- Backup/restore functionality
- Profile management (different configs for different scenarios)
- Cloud sync for settings
- Integration with other Wayland compositors

## Dependencies
### Required
- Hyprland (obviously)
- Wayland libraries
- D-Bus for system integration

### Optional
- Polkit for privileged operations
- NetworkManager for network settings
- PulseAudio/PipeWire for audio settings

## Similar Projects to Study
- COSMIC Settings (Pop!_OS)
- GNOME Settings
- KDE System Settings
- Sway Settings (if exists)
- Wayfire Config Manager

## Notes
- Prioritize stability over features
- Keep it lightweight and fast
- Respect user's existing configurations
- Provide clear documentation
- Consider i18n from the start