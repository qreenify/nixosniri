//! Wonderland Theme System
//!
//! Loads themes from ~/.config/theme/themes/ and provides Iced theming.

mod color;
mod loader;
pub mod iced_theme;

pub use color::Color;
pub use loader::{Theme, ThemeLoader, ThemeError};
pub use iced_theme::{WonderlandTheme, ContainerClass, ButtonClass, TextClass, TextInputClass};

/// Default theme directory
pub const THEME_DIR: &str = ".config/theme/themes";

/// Current theme symlink location
pub const CURRENT_THEME: &str = ".config/theme/current/theme";
