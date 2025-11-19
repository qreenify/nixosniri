//! Theme loading from disk

use crate::color::Color;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// A complete theme definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub path: PathBuf,

    // Core colors (from waybar.css)
    pub foreground: Color,
    pub background: Color,

    // Extended palette (derived or from extended config)
    pub primary: Color,
    pub secondary: Color,
    pub surface: Color,
    pub error: Color,
    pub warning: Color,
    pub success: Color,

    // UI specific
    pub border: Color,
    pub border_active: Color,
    pub text_muted: Color,
}

impl Theme {
    /// Derive extended colors from foreground/background
    fn derive_extended_colors(fg: Color, bg: Color) -> Self {
        // Determine if dark or light theme
        let is_dark = (bg.r + bg.g + bg.b) / 3.0 < 0.5;

        let (surface, text_muted) = if is_dark {
            (bg.lighten(0.08), fg.darken(0.3))
        } else {
            (bg.darken(0.05), fg.lighten(0.3))
        };

        Theme {
            name: String::new(),
            path: PathBuf::new(),
            foreground: fg,
            background: bg,
            primary: fg,  // Use foreground as primary accent
            secondary: fg.darken(0.2),
            surface,
            error: Color::from_hex("#f38ba8").unwrap_or(fg),
            warning: Color::from_hex("#f9e2af").unwrap_or(fg),
            success: Color::from_hex("#a6e3a1").unwrap_or(fg),
            border: if is_dark { bg.lighten(0.15) } else { bg.darken(0.15) },
            border_active: fg,
            text_muted,
        }
    }
}

/// Theme loader with directory scanning and hot-reload support
pub struct ThemeLoader {
    themes_dir: PathBuf,
    current_link: PathBuf,
}

impl ThemeLoader {
    /// Create a new theme loader
    pub fn new() -> Result<Self, ThemeError> {
        let home = std::env::var("HOME").map_err(|_| ThemeError::NoHomeDir)?;
        let home = PathBuf::from(home);

        Ok(Self {
            themes_dir: home.join(crate::THEME_DIR),
            current_link: home.join(crate::CURRENT_THEME),
        })
    }

    /// Create with custom paths
    pub fn with_paths(themes_dir: PathBuf, current_link: PathBuf) -> Self {
        Self {
            themes_dir,
            current_link,
        }
    }

    /// List all available themes
    pub fn list_themes(&self) -> Result<Vec<String>, ThemeError> {
        let mut themes = Vec::new();

        let entries = std::fs::read_dir(&self.themes_dir)
            .map_err(|e| ThemeError::Io(self.themes_dir.clone(), e))?;

        for entry in entries {
            let entry = entry.map_err(|e| ThemeError::Io(self.themes_dir.clone(), e))?;
            if entry.path().is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    themes.push(name.to_string());
                }
            }
        }

        themes.sort();
        Ok(themes)
    }

    /// Get the currently active theme name
    pub fn current_theme_name(&self) -> Result<String, ThemeError> {
        let target = std::fs::read_link(&self.current_link)
            .map_err(|e| ThemeError::Io(self.current_link.clone(), e))?;

        // Extract theme name from path
        target
            .file_name()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string())
            .ok_or(ThemeError::InvalidThemePath(target))
    }

    /// Load the current theme
    pub fn load_current(&self) -> Result<Theme, ThemeError> {
        let name = self.current_theme_name()?;
        self.load_theme(&name)
    }

    /// Load a theme by name
    pub fn load_theme(&self, name: &str) -> Result<Theme, ThemeError> {
        let theme_path = self.themes_dir.join(name);

        if !theme_path.exists() {
            return Err(ThemeError::NotFound(name.to_string()));
        }

        // Parse waybar.css for core colors
        let waybar_css = theme_path.join("waybar.css");
        let (foreground, background) = if waybar_css.exists() {
            parse_waybar_css(&waybar_css)?
        } else {
            // Default fallback
            (
                Color::from_hex("#cdd6f4").unwrap(),
                Color::from_hex("#1e1e2e").unwrap(),
            )
        };

        // Build theme with derived colors
        let mut theme = Theme::derive_extended_colors(foreground, background);
        theme.name = name.to_string();
        theme.path = theme_path;

        // TODO: Load extended colors from theme.toml if it exists
        // This allows themes to override derived colors

        Ok(theme)
    }

    /// Get path to theme backgrounds directory
    pub fn backgrounds_dir(&self, theme_name: &str) -> PathBuf {
        self.themes_dir.join(theme_name).join("backgrounds")
    }
}

impl Default for ThemeLoader {
    fn default() -> Self {
        Self::new().expect("Failed to create theme loader")
    }
}

/// Parse waybar.css for foreground and background colors
fn parse_waybar_css(path: &Path) -> Result<(Color, Color), ThemeError> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| ThemeError::Io(path.to_path_buf(), e))?;

    let mut foreground = None;
    let mut background = None;

    for line in content.lines() {
        let line = line.trim();

        if line.starts_with("@define-color foreground") {
            if let Some(color) = extract_color(line) {
                foreground = Some(Color::from_hex(&color)?);
            }
        } else if line.starts_with("@define-color background") {
            if let Some(color) = extract_color(line) {
                background = Some(Color::from_hex(&color)?);
            }
        }
    }

    Ok((
        foreground.ok_or(ThemeError::MissingColor("foreground".to_string()))?,
        background.ok_or(ThemeError::MissingColor("background".to_string()))?,
    ))
}

/// Extract color value from CSS @define-color line
fn extract_color(line: &str) -> Option<String> {
    // Format: @define-color name #hexcolor;
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 3 {
        Some(parts[2].trim_end_matches(';').to_string())
    } else {
        None
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ThemeError {
    #[error("HOME environment variable not set")]
    NoHomeDir,

    #[error("Theme not found: {0}")]
    NotFound(String),

    #[error("Invalid theme path: {0}")]
    InvalidThemePath(PathBuf),

    #[error("Missing required color: {0}")]
    MissingColor(String),

    #[error("IO error at {0}: {1}")]
    Io(PathBuf, std::io::Error),

    #[error("Color parse error: {0}")]
    ColorParse(#[from] crate::color::ColorError),
}
