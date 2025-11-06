use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use anyhow::Result;

pub mod hyprland;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub appearance: AppearanceConfig,
    pub display: DisplayConfig,
    pub input: InputConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceConfig {
    pub theme: String,
    pub icon_theme: String,
    pub cursor_theme: String,
    pub font: String,
    pub font_size: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    pub resolution: String,
    pub refresh_rate: i32,
    pub scale: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputConfig {
    pub mouse_sensitivity: f32,
    pub keyboard_layout: String,
    pub touchpad_enabled: bool,
}

pub struct ConfigManager {
    config_path: PathBuf,
    hyprland_config_path: PathBuf,
    app_config: AppConfig,
}

impl ConfigManager {
    pub fn new() -> Self {
        let config_path = Self::get_config_path();
        let hyprland_config_path = Self::get_hyprland_config_path();
        let app_config = Self::load_config(&config_path).unwrap_or_default();

        Self {
            config_path,
            hyprland_config_path,
            app_config,
        }
    }

    fn get_config_path() -> PathBuf {
        let mut path = dirs::config_dir().expect("Failed to get config directory");
        path.push("hyprland-settings");
        path.push("config.toml");
        path
    }

    fn get_hyprland_config_path() -> PathBuf {
        let mut path = dirs::config_dir().expect("Failed to get config directory");
        path.push("hypr");
        path.push("hyprland.conf");
        path
    }

    fn load_config(path: &PathBuf) -> Result<AppConfig> {
        if path.exists() {
            let contents = fs::read_to_string(path)?;
            Ok(toml::from_str(&contents)?)
        } else {
            Ok(AppConfig::default())
        }
    }

    pub fn save_config(&self) -> Result<()> {
        let config_dir = self.config_path.parent().unwrap();
        if !config_dir.exists() {
            fs::create_dir_all(config_dir)?;
        }

        let contents = toml::to_string_pretty(&self.app_config)?;
        fs::write(&self.config_path, contents)?;
        Ok(())
    }

    pub fn get_hyprland_config(&self) -> Result<hyprland::HyprlandConfig> {
        hyprland::HyprlandConfig::load(&self.hyprland_config_path)
    }

    pub fn save_hyprland_config(&self, config: &hyprland::HyprlandConfig) -> Result<()> {
        config.save(&self.hyprland_config_path)
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            appearance: AppearanceConfig {
                theme: "Adwaita-dark".to_string(),
                icon_theme: "Papirus-Dark".to_string(),
                cursor_theme: "Adwaita".to_string(),
                font: "Inter".to_string(),
                font_size: 11,
            },
            display: DisplayConfig {
                resolution: "1920x1080".to_string(),
                refresh_rate: 60,
                scale: 1.0,
            },
            input: InputConfig {
                mouse_sensitivity: 1.0,
                keyboard_layout: "us".to_string(),
                touchpad_enabled: true,
            },
        }
    }
}