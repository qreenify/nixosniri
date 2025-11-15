# Omarchy Theme System

## How Themes Work

### Directory Structure
```
~/.config/omarchy/
├── themes/              # All available themes
│   ├── tokyo-night/
│   ├── catppuccin/
│   ├── gruvbox/
│   └── ...
└── current/
    └── theme -> ../themes/tokyo-night  # Symlink to active theme
```

### Theme Selection Process

1. **Themes are stored in**: `~/.config/omarchy/themes/`
2. **Current theme is a symlink**: `~/.config/omarchy/current/theme` points to the active theme
3. **Theme files reference the current symlink**: Applications load theme files from `~/.config/omarchy/current/theme/<app>.css`

### Example: Walker Theme

Walker's theme CSS (located in `.local/share/omarchy/default/walker/themes/omarchy-default/style.css`):

```css
@import "../../../../../../../.config/omarchy/current/theme/walker.css";
```

This import path resolves to:
- `~/.local/share/omarchy/default/walker/themes/omarchy-default/style.css`
- Goes up to root, then down to `~/.config/omarchy/current/theme/walker.css`
- Which is actually `~/.config/omarchy/themes/<theme-name>/walker.css` (via symlink)

### Theme Application Script

`omarchy-theme-set <theme-name>` does:

1. Creates symlink: `ln -nsf "$THEMES_DIR/$THEME_NAME" "$CURRENT_THEME_DIR"`
2. Changes wallpaper
3. Restarts components: waybar, swayosd, hyprland, btop, mako
4. Updates terminal, gnome, browser, vscode, cursor, obsidian themes
5. Calls theme-set hook

### Theme Structure

Each theme directory contains:
- `walker.css` - Walker launcher colors
- `waybar.css` - Waybar statusbar colors
- `hyprland.conf` - Hyprland compositor theme
- `hyprlock.conf` - Lock screen theme
- `mako.ini` - Notification theme
- `alacritty.toml` - Terminal theme
- `kitty.conf` - Kitty terminal theme
- `btop.theme` - System monitor theme
- `neovim.lua` - Neovim colorscheme
- `backgrounds/` - Wallpapers for this theme

## NixOS Compatibility Issues

### Problem 1: Read-only Nix Store

Home-manager creates symlinks from `~/.config/` to the **read-only nix store**.

When we do:
```nix
home.file.".config/omarchy/current/theme".source = ../omarchy/themes/tokyo-night;
```

This creates:
```
~/.config/omarchy/current/theme -> /nix/store/xxxxx-tokyo-night
```

**Issue**: Omarchy expects to change this symlink dynamically with `omarchy-theme-set`, but can't modify nix store symlinks.

### Problem 2: Theme Directory Needs to be Writable

Omarchy's `omarchy-theme-set` script needs to:
```bash
ln -nsf "$THEMES_DIR/$THEME_NAME" "$CURRENT_THEME_DIR"
```

This fails if the directory is managed by home-manager.

### Problem 3: Arch-specific Dependencies

Some omarchy scripts expect:
- `uwsm-app` (Arch session manager)
- `elephant` (Unknown dependency)
- Walker options that don't exist in NixOS walker package (`--minheight`, `--maxheight`)

## Solutions for NixOS

### Option 1: Manual Theme Directory (Recommended)

1. **Don't manage themes via home-manager**
2. **Copy themes once** to `~/.config/omarchy/themes/` as regular files
3. **Let omarchy manage** the symlink at `~/.config/omarchy/current/theme`
4. **User can switch themes** using omarchy menu or `omarchy-theme-set`

### Option 2: Home-manager with Activation Script

1. Use home-manager to deploy themes
2. Add activation script to:
   - Copy themes from nix store to `~/.config/omarchy/themes/`
   - Create initial symlink to default theme
   - Make directories writable

### Option 3: Hybrid Approach

1. Deploy omarchy scripts via home-manager (bin, default)
2. **Manually install themes** once: `cp -r ~/git/omarchy/themes ~/.config/omarchy/`
3. Let user manage theme switching with omarchy commands
4. Only update themes when explicitly requested

## Recommended Approach for User

Given the user wants omarchy theming to work properly:

1. **Copy themes manually** (one-time):
   ```bash
   mkdir -p ~/.config/omarchy
   cp -r ~/git/omarchy/themes ~/.config/omarchy/
   ```

2. **Set initial theme**:
   ```bash
   omarchy-theme-set tokyo-night
   ```

3. **Keep scripts managed by NixOS** (already done)

4. **Let omarchy handle theme switching** (via its menu or commands)

This allows:
- ✅ Themes can be switched dynamically
- ✅ Symlink can be updated
- ✅ All omarchy theme commands work
- ✅ NixOS still manages the scripts
- ⚠️ Themes are not in git/reproducible (but that's okay - they're user preference)
