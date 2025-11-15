# NixOS-Compatible Omarchy Setup Plan

## Current Status

### What's Working ✅
- Omarchy scripts deployed to `~/.local/share/omarchy/bin/`
- Walker package installed
- Walker config deployed to `~/.config/walker/`
- `omarchy` alias configured in nushell

### What's Broken ❌
- Walker/omarchy menu is transparent (no theme colors)
- Theme symlink doesn't exist at `~/.config/omarchy/current/theme`
- Themes not properly accessible

## Root Cause Analysis

Omarchy's theme system requires:
1. Writable `~/.config/omarchy/themes/` directory with theme files
2. Writable symlink at `~/.config/omarchy/current/theme` pointing to active theme
3. Theme CSS files to be loaded via this symlink path

Home-manager creates **read-only symlinks** to nix store, which breaks:
- Dynamic theme switching (can't update symlink)
- Theme installation/removal
- Omarchy theme management commands

## Proposed Solution

### Hybrid Approach: NixOS + Manual Themes

**NixOS manages (via home-manager)**:
- ✅ Omarchy scripts (`~/.local/share/omarchy/bin/`)
- ✅ Omarchy default files (`~/.local/share/omarchy/default/`)
- ✅ Walker config (`~/.config/walker/`)
- ✅ Shell aliases

**User manages manually**:
- 👤 Theme files (`~/.config/omarchy/themes/`)
- 👤 Current theme symlink (`~/.config/omarchy/current/theme`)
- 👤 Theme switching via omarchy commands

### Why This Works

✅ Themes are user preference (like wallpapers), not system config
✅ Omarchy commands can modify theme symlink
✅ Theme switching works dynamically
✅ NixOS still manages the core omarchy system
✅ Reproducible: themes can be copied from git when needed

## Step-by-Step Setup

### Step 1: Update NixOS Config

Current `~/.config/nixos/modules/home.nix`:
```nix
# === Omarchy Installation ===
home.file.".local/share/omarchy/bin".source = ../omarchy/bin;
home.file.".local/share/omarchy/default".source = ../omarchy/default;
home.file.".config/walker".source = ../omarchy/config/walker;
home.file.".config/walker".recursive = true;
home.file.".config/elephant".source = ../omarchy/config/elephant;
```

**REMOVE** these lines (they don't work):
```nix
home.file.".local/share/omarchy/themes".source = ../omarchy/themes;  # ❌ DELETE
home.file.".config/omarchy/current/theme".source = ../omarchy/themes/tokyo-night;  # ❌ DELETE
```

**Keep only**:
```nix
# === Omarchy Installation ===
home.file.".local/share/omarchy/bin".source = ../omarchy/bin;
home.file.".local/share/omarchy/default".source = ../omarchy/default;
home.file.".config/walker".source = ../omarchy/config/walker;
home.file.".config/walker".recursive = true;
home.file.".config/elephant".source = ../omarchy/config/elephant;
```

### Step 2: Manual Theme Installation

After NixOS rebuild, run these commands **once**:

```bash
# Copy themes to config directory
mkdir -p ~/.config/omarchy
cp -r ~/git/omarchy/themes ~/.config/omarchy/

# Set initial theme (tokyo-night)
mkdir -p ~/.config/omarchy/current
ln -nsf ~/.config/omarchy/themes/tokyo-night ~/.config/omarchy/current/theme

# Verify symlink
ls -la ~/.config/omarchy/current/theme
# Should show: theme -> /home/qreenify/.config/omarchy/themes/tokyo-night
```

### Step 3: Test

```bash
# Test walker with theme
walker

# Test omarchy menu
omarchy

# Try changing theme
omarchy-theme-set catppuccin
```

## Theme Management

### Switching Themes

```bash
# Via omarchy menu
omarchy
# → Navigate to "Style"
# → Select theme

# Via command line
omarchy-theme-set tokyo-night
omarchy-theme-set gruvbox
omarchy-theme-set catppuccin
```

### Available Themes

- catppuccin (dark)
- catppuccin-latte (light)
- everforest
- flexoki-light (light)
- gruvbox
- kanagawa
- matte-black
- nord
- osaka-jade
- ristretto
- rose-pine (light)
- tokyo-night (recommended)

### Updating Themes

If omarchy repo is updated:

```bash
# Backup current theme selection
current_theme=$(basename "$(realpath ~/.config/omarchy/current/theme)")

# Update themes
rm -rf ~/.config/omarchy/themes
cp -r ~/git/omarchy/themes ~/.config/omarchy/

# Restore theme selection
ln -nsf ~/.config/omarchy/themes/$current_theme ~/.config/omarchy/current/theme
```

## Alternative: NixOS Activation Script

If you want themes managed by NixOS, use home-manager activation:

```nix
home.activation.omarchyThemes = lib.hm.dag.entryAfter ["writeBoundary"] ''
  # Copy themes to writable location
  $DRY_RUN_CMD mkdir -p $HOME/.config/omarchy
  $DRY_RUN_CMD cp -rf ${../omarchy/themes} $HOME/.config/omarchy/themes
  $DRY_RUN_CMD chmod -R u+w $HOME/.config/omarchy/themes

  # Create default theme symlink if doesn't exist
  if [ ! -L $HOME/.config/omarchy/current/theme ]; then
    $DRY_RUN_CMD mkdir -p $HOME/.config/omarchy/current
    $DRY_RUN_CMD ln -nsf $HOME/.config/omarchy/themes/tokyo-night $HOME/.config/omarchy/current/theme
  fi
'';
```

**Pros**: Automatic theme installation
**Cons**: Rebuilds will overwrite manual theme changes

## Recommendation

**Use manual theme installation** (Step 2 above).

**Reasoning**:
- Themes are personal preference, not system state
- Omarchy is designed for dynamic theme switching
- Keeps NixOS config simpler
- Allows omarchy theme commands to work properly
- User has full control over themes

## Documentation Location

Save this documentation in `~/.config/nixos/docs/omarchy-setup.md` for reference.

## Summary

1. ✅ Keep current NixOS config for scripts/walker
2. ❌ Remove theme-related home-manager lines
3. 👤 Manually copy themes to `~/.config/omarchy/themes/`
4. 👤 Create symlink to select default theme
5. 🎨 Use omarchy commands for theme switching
6. 📝 Document this process in nixos config repo

This gives you:
- Working omarchy menu with proper colors
- Dynamic theme switching
- NixOS-managed scripts
- Best of both worlds!
