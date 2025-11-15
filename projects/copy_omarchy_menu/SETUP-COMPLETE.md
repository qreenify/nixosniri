# Omarchy Theme System - READY TO USE

## What I've Created

### 1. Theme Setup Script ✅
**Location**: `~/.config/nixos/scripts/omarchy-theme-setup`

One-time setup script that:
- Copies all themes from `~/git/omarchy/themes/` to `~/.config/omarchy/themes/`
- Sets up the default theme (tokyo-night)
- Creates the required symlink structure

### 2. Theme Browser GUI ✅
**Location**: `~/.config/nixos/scripts/omarchy-theme-browser`

A beautiful GTK application that shows:
- **Left panel (1/3)**: List of all available themes with current theme indicator (●)
- **Right panel (2/3)**: Preview image (preview.png) of selected theme
- **Apply button**: Click to switch to the selected theme

### 3. NixOS Integration ✅
**Updated**: `~/.config/nixos/modules/home.nix`

Changes made:
- ❌ Removed broken theme deployment lines
- ✅ Added Python with GTK support
- ✅ Added `theme` command alias
- ✅ Theme browser and setup scripts deployed via home-manager

### 4. Shell Command ✅
**New command**: `theme`

Launches the graphical theme browser.

## How to Use

### Step 1: Initial Setup (Run Once)

```bash
# 1. Rebuild NixOS to get the new scripts and packages
rebuild

# 2. Run the theme setup script (one-time)
~/.script/omarchy-theme-setup
```

This will:
- Copy all 12 themes to `~/.config/omarchy/themes/`
- Set tokyo-night as the default theme
- Create the symlink at `~/.config/omarchy/current/theme`

### Step 2: Browse and Switch Themes

```bash
# Open the graphical theme browser
theme
```

Or from a new terminal after rebuild:
```bash
theme
```

The theme browser will show:
- All available themes on the left
- Preview image on the right
- Current theme marked with ● indicator
- Click a theme to see its preview
- Click "Apply Theme" to switch

### Available Themes

1. **catppuccin** (dark, purple-pink) ⭐
2. **catppuccin-latte** (light)
3. **everforest** (dark green)
4. **flexoki-light** (light, warm)
5. **gruvbox** (dark, retro) ⭐
6. **kanagawa** (dark, japanese)
7. **matte-black** (dark, minimal)
8. **nord** (dark, blue) ⭐
9. **osaka-jade** (dark, jade green)
10. **ristretto** (dark, coffee)
11. **rose-pine** (light, pink)
12. **tokyo-night** (dark, blue) ⭐ DEFAULT

## Commands Reference

```bash
# GUI theme browser (recommended)
theme

# Setup themes (one-time)
~/.script/omarchy-theme-setup

# Command-line theme switching (if you know the name)
omarchy-theme-set tokyo-night
omarchy-theme-set gruvbox
omarchy-theme-set catppuccin

# List available themes
ls ~/.config/omarchy/themes/

# Check current theme
basename "$(readlink ~/.config/omarchy/current/theme)"
```

## How It Works

### Architecture

```
~/.config/omarchy/
├── themes/                    # All theme files (12 themes)
│   ├── tokyo-night/
│   │   ├── walker.css
│   │   ├── waybar.css
│   │   ├── preview.png       # Preview image shown in GUI
│   │   └── ...
│   ├── catppuccin/
│   └── ...
└── current/
    └── theme -> ../themes/tokyo-night  # Symlink to active theme
```

### When You Switch Themes

1. Click theme in GUI → See preview
2. Click "Apply Theme" button
3. Runs: `omarchy-theme-set <theme-name>`
4. Updates symlink: `~/.config/omarchy/current/theme`
5. Restarts: waybar, hyprland, mako, etc.
6. New theme applied! 🎨

### Walker Integration

Walker's theme CSS imports:
```css
@import "~/.config/omarchy/current/theme/walker.css"
```

This resolves to the active theme's colors:
- Background: `@base`
- Text: `@text`
- Border: `@border`
- Selected: `@selected-text`

**No more transparency!** 🎉

## Troubleshooting

### "Themes Not Found" Error
Run the setup script:
```bash
~/.script/omarchy-theme-setup
```

### Theme Browser Won't Launch
Check Python GTK is installed:
```bash
python3 -c "import gi; gi.require_version('Gtk', '3.0'); from gi.repository import Gtk; print('OK')"
```

If error, rebuild NixOS:
```bash
rebuild
```

### Theme Switching Doesn't Work
Verify omarchy-theme-set is in PATH:
```bash
which omarchy-theme-set
# Should show: /home/qreenify/.local/share/omarchy/bin/omarchy-theme-set
```

### Walker Still Transparent
Check the symlink exists:
```bash
ls -la ~/.config/omarchy/current/theme
# Should show: theme -> /home/qreenify/.config/omarchy/themes/tokyo-night
```

If missing, run setup:
```bash
~/.script/omarchy-theme-setup
```

## What's Next

After you run `rebuild` and `~/.script/omarchy-theme-setup`:

1. ✅ Omarchy menu will have proper colors (no more transparency)
2. ✅ Walker will have themed colors
3. ✅ You can browse themes with preview images
4. ✅ Switch themes with one click
5. ✅ All omarchy theme commands work

Just type `theme` to get started! 🚀
