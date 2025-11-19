//! Color parsing and conversion utilities

use serde::{Deserialize, Serialize};

/// RGBA color representation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Parse hex color string (#RGB, #RGBA, #RRGGBB, #RRGGBBAA)
    pub fn from_hex(hex: &str) -> Result<Self, ColorError> {
        let hex = hex.trim_start_matches('#');

        let (r, g, b, a) = match hex.len() {
            3 => {
                let r = u8::from_str_radix(&hex[0..1], 16)? * 17;
                let g = u8::from_str_radix(&hex[1..2], 16)? * 17;
                let b = u8::from_str_radix(&hex[2..3], 16)? * 17;
                (r, g, b, 255)
            }
            4 => {
                let r = u8::from_str_radix(&hex[0..1], 16)? * 17;
                let g = u8::from_str_radix(&hex[1..2], 16)? * 17;
                let b = u8::from_str_radix(&hex[2..3], 16)? * 17;
                let a = u8::from_str_radix(&hex[3..4], 16)? * 17;
                (r, g, b, a)
            }
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16)?;
                let g = u8::from_str_radix(&hex[2..4], 16)?;
                let b = u8::from_str_radix(&hex[4..6], 16)?;
                (r, g, b, 255)
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16)?;
                let g = u8::from_str_radix(&hex[2..4], 16)?;
                let b = u8::from_str_radix(&hex[4..6], 16)?;
                let a = u8::from_str_radix(&hex[6..8], 16)?;
                (r, g, b, a)
            }
            _ => return Err(ColorError::InvalidLength(hex.len())),
        };

        Ok(Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        })
    }

    /// Convert to Iced color
    pub fn to_iced(&self) -> iced::Color {
        iced::Color::from_rgba(self.r, self.g, self.b, self.a)
    }

    /// Lighten the color by a factor (0.0 - 1.0)
    pub fn lighten(&self, factor: f32) -> Self {
        Self {
            r: (self.r + (1.0 - self.r) * factor).min(1.0),
            g: (self.g + (1.0 - self.g) * factor).min(1.0),
            b: (self.b + (1.0 - self.b) * factor).min(1.0),
            a: self.a,
        }
    }

    /// Darken the color by a factor (0.0 - 1.0)
    pub fn darken(&self, factor: f32) -> Self {
        Self {
            r: (self.r * (1.0 - factor)).max(0.0),
            g: (self.g * (1.0 - factor)).max(0.0),
            b: (self.b * (1.0 - factor)).max(0.0),
            a: self.a,
        }
    }

    /// Set alpha value
    pub fn with_alpha(&self, a: f32) -> Self {
        Self { a, ..*self }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ColorError {
    #[error("Invalid hex color length: {0}")]
    InvalidLength(usize),
    #[error("Invalid hex digit: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_hex_colors() {
        let color = Color::from_hex("#ff0000").unwrap();
        assert_eq!(color.r, 1.0);
        assert_eq!(color.g, 0.0);
        assert_eq!(color.b, 0.0);

        let color = Color::from_hex("#1a1b26").unwrap();
        assert!((color.r - 0.102).abs() < 0.01);
    }
}
