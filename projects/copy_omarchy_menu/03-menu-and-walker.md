# Omarchy Menu System and Walker Integration

## Menu Entry Point

```
omarchy -> omarchy-menu -> omarchy-launch-walker
```

### Flow:

1. **User types**: `omarchy`
2. **Calls**: `~/.local/share/omarchy/bin/omarchy-menu`
3. **Which calls**: `omarchy-launch-walker --dmenu --width 295 --minheight 1 --maxheight 600 -p Go…`
4. **Walker displays** menu with options

## Walker Integration

### Original Arch Implementation

`omarchy-launch-walker` (Arch version):
```bash
# Ensure elephant is running
if ! pgrep -x elephant > /dev/null; then
  setsid uwsm-app -- elephant &
fi

# Ensure walker service is running
if ! pgrep -f "walker --gapplication-service" > /dev/null; then
  setsid uwsm-app -- walker --gapplication-service &
fi

exec walker --width 644 --maxheight 300 --minheight 300 "$@"
```

**Dependencies**:
- `elephant` - Unknown daemon (not in NixOS)
- `uwsm-app` - Arch Universal Wayland Session Manager (not in NixOS)
- Walker options: `--maxheight`, `--minheight` (not in NixOS walker)

### NixOS Walker Differences

NixOS walker package supports:
- `--width` ✅
- `--height` ✅ (single value, not min/max)
- `--dmenu` ✅
- `-p` (prompt) ✅

Does NOT support:
- `--minheight` ❌
- `--maxheight` ❌
- May have different version/features

## Walker Configuration

### Config Location
`~/.config/walker/config.toml`

Key settings:
```toml
theme = "omarchy-default"
additional_theme_location = "~/.local/share/omarchy/default/walker/themes/"
```

### Theme Loading Path

1. Walker looks for theme in: `~/.local/share/omarchy/default/walker/themes/omarchy-default/`
2. Loads: `style.css` and `layout.xml`
3. `style.css` imports: `@import "../../../../../../../.config/omarchy/current/theme/walker.css"`
4. This resolves to: `~/.config/omarchy/current/theme/walker.css`
5. Which is symlinked to: `~/.config/omarchy/themes/<current-theme>/walker.css`

### Why Walker Was Transparent

The transparency issue happened because:

1. ❌ `~/.config/omarchy/current/theme` didn't exist (or wasn't a proper symlink)
2. ❌ The CSS import failed silently
3. ❌ No color variables were defined (`@base`, `@text`, `@border`, etc.)
4. ❌ Walker used default transparent/minimal styling

### Theme CSS Variables

From `tokyo-night/walker.css`:
```css
@define-color selected-text #7dcfff;
@define-color text #cfc9c2;
@define-color base #1a1b26;
@define-color border #33ccff;
@define-color foreground #cfc9c2;
@define-color background #1a1b26;
```

These are used in `style.css`:
```css
.box-wrapper {
  background: alpha(@base, 0.95);
  padding: 20px;
  border: 2px solid @border;
}
```

## Menu Structure

When `omarchy-menu` runs, it shows:

```
󰀻  Apps
󰧑  Learn
󱓞  Trigger
  Style
  Setup
󰉉  Install
󰭌  Remove
  Update
  About
  System
```

Each option triggers different omarchy scripts:
- Apps → Application launcher
- Style → Theme selection
- Setup → System configuration
- Install → Package installation
- etc.

## NixOS Adaptations Needed

### 1. Fix omarchy-launch-walker

Remove Arch dependencies:
```bash
#!/usr/bin/env bash
# Filter out unsupported --minheight/--maxheight
args=()
for arg in "$@"; do
  case "$arg" in
    --minheight|--maxheight) skip_next=true ;;
    *) [[ "$skip_next" != true ]] && args+=("$arg") ;;
  esac
done

exec walker --width 644 --height 300 "${args[@]}"
```

### 2. Ensure Theme Symlink Exists

Before walker can load themes:
```bash
mkdir -p ~/.config/omarchy/current
ln -nsf ~/.config/omarchy/themes/tokyo-night ~/.config/omarchy/current/theme
```

### 3. Make Walker Config Writable

Walker needs to create `default.css` in `~/.config/walker/themes/`

Home-manager config:
```nix
home.file.".config/walker".source = ../omarchy/config/walker;
home.file.".config/walker".recursive = true;  # Copy, don't symlink
```

## Summary

For omarchy to work on NixOS:

1. ✅ Scripts deployed via home-manager (with NixOS adaptations)
2. ✅ Walker config copied (writable) via home-manager
3. ⚠️ **Themes must be manually copied** to `~/.config/omarchy/themes/`
4. ⚠️ **Symlink must be manually created** to select active theme
5. ✅ Then walker loads colors from theme CSS
6. ✅ Menu system works with proper styling
