use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HyprlandConfig {
    // General
    pub general: GeneralConfig,

    // Decoration
    pub decoration: DecorationConfig,

    // Animations
    pub animations: AnimationsConfig,

    // Input
    pub input: InputConfig,

    // Binds
    pub binds: Vec<Keybind>,

    // Window Rules
    pub windowrules: Vec<WindowRule>,

    // Monitor configuration
    pub monitors: Vec<MonitorConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub gaps_in: i32,
    pub gaps_out: i32,
    pub border_size: i32,
    pub col_active_border: String,
    pub col_inactive_border: String,
    pub layout: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecorationConfig {
    pub rounding: i32,
    pub blur: bool,
    pub blur_size: i32,
    pub blur_passes: i32,
    pub drop_shadow: bool,
    pub shadow_range: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationsConfig {
    pub enabled: bool,
    pub bezier: Vec<String>,
    pub animation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputConfig {
    pub kb_layout: String,
    pub follow_mouse: i32,
    pub sensitivity: f32,
    pub touchpad: TouchpadConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TouchpadConfig {
    pub natural_scroll: bool,
    pub disable_while_typing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keybind {
    pub modifiers: Vec<String>,
    pub key: String,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowRule {
    pub rule: String,
    pub pattern: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorConfig {
    pub name: String,
    pub resolution: String,
    pub position: String,
    pub scale: f32,
}

impl HyprlandConfig {
    pub fn load(path: &PathBuf) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }

        // Parse the Hyprland config file
        // This is a simplified parser - real implementation would need
        // to handle the full Hyprland config syntax
        let contents = fs::read_to_string(path)?;
        Self::parse_config(&contents)
    }

    pub fn save(&self, path: &PathBuf) -> Result<()> {
        let contents = self.to_config_string();
        fs::write(path, contents)?;
        Ok(())
    }

    fn parse_config(contents: &str) -> Result<Self> {
        // This is a simplified parser
        // A real implementation would need a proper parser for Hyprland's config format
        let mut config = Self::default();

        for line in contents.lines() {
            let line = line.trim();

            // Skip comments and empty lines
            if line.starts_with('#') || line.is_empty() {
                continue;
            }

            // Parse different config sections
            // This is very simplified - real parsing would be more complex
            if line.starts_with("general:") {
                // Parse general settings
            } else if line.starts_with("decoration:") {
                // Parse decoration settings
            } else if line.starts_with("bind") {
                // Parse keybindings
            }
            // ... etc
        }

        Ok(config)
    }

    fn to_config_string(&self) -> String {
        let mut config = String::new();

        // General section
        config.push_str("general {\n");
        config.push_str(&format!("    gaps_in = {}\n", self.general.gaps_in));
        config.push_str(&format!("    gaps_out = {}\n", self.general.gaps_out));
        config.push_str(&format!("    border_size = {}\n", self.general.border_size));
        config.push_str(&format!("    col.active_border = {}\n", self.general.col_active_border));
        config.push_str(&format!("    col.inactive_border = {}\n", self.general.col_inactive_border));
        config.push_str(&format!("    layout = {}\n", self.general.layout));
        config.push_str("}\n\n");

        // Decoration section
        config.push_str("decoration {\n");
        config.push_str(&format!("    rounding = {}\n", self.decoration.rounding));
        config.push_str(&format!("    blur = {}\n", self.decoration.blur));
        if self.decoration.blur {
            config.push_str(&format!("    blur_size = {}\n", self.decoration.blur_size));
            config.push_str(&format!("    blur_passes = {}\n", self.decoration.blur_passes));
        }
        config.push_str(&format!("    drop_shadow = {}\n", self.decoration.drop_shadow));
        config.push_str(&format!("    shadow_range = {}\n", self.decoration.shadow_range));
        config.push_str("}\n\n");

        // Input section
        config.push_str("input {\n");
        config.push_str(&format!("    kb_layout = {}\n", self.input.kb_layout));
        config.push_str(&format!("    follow_mouse = {}\n", self.input.follow_mouse));
        config.push_str(&format!("    sensitivity = {}\n", self.input.sensitivity));
        config.push_str("\n    touchpad {\n");
        config.push_str(&format!("        natural_scroll = {}\n", self.input.touchpad.natural_scroll));
        config.push_str(&format!("        disable_while_typing = {}\n", self.input.touchpad.disable_while_typing));
        config.push_str("    }\n");
        config.push_str("}\n\n");

        // Keybindings
        for bind in &self.binds {
            let mods = bind.modifiers.join(" ");
            config.push_str(&format!("bind = {}, {}, {}\n", mods, bind.key, bind.action));
        }

        config
    }
}

impl Default for HyprlandConfig {
    fn default() -> Self {
        Self {
            general: GeneralConfig {
                gaps_in: 5,
                gaps_out: 10,
                border_size: 2,
                col_active_border: "rgba(33ccffee) rgba(00ff99ee) 45deg".to_string(),
                col_inactive_border: "rgba(595959aa)".to_string(),
                layout: "dwindle".to_string(),
            },
            decoration: DecorationConfig {
                rounding: 10,
                blur: true,
                blur_size: 3,
                blur_passes: 1,
                drop_shadow: true,
                shadow_range: 4,
            },
            animations: AnimationsConfig {
                enabled: true,
                bezier: vec!["myBezier, 0.05, 0.9, 0.1, 1.05".to_string()],
                animation: vec![
                    "windows, 1, 7, myBezier".to_string(),
                    "windowsOut, 1, 7, default, popin 80%".to_string(),
                ],
            },
            input: InputConfig {
                kb_layout: "us".to_string(),
                follow_mouse: 1,
                sensitivity: 0.0,
                touchpad: TouchpadConfig {
                    natural_scroll: false,
                    disable_while_typing: true,
                },
            },
            binds: vec![],
            windowrules: vec![],
            monitors: vec![],
        }
    }
}