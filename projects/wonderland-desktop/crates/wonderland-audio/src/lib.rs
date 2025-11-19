//! PipeWire audio control
//!
//! Monitor audio streams and control volume.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Known application identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppId {
    YouTube,
    AppleMusic,
    Twitch,
    Plex,
    Jellyfin,
    Discord,
    Browser,
    Unknown,
}

impl AppId {
    /// Detect app from process command line
    pub fn from_cmdline(cmdline: &str) -> Self {
        let lower = cmdline.to_lowercase();

        if lower.contains("youtube") || lower.contains("music.youtube") {
            AppId::YouTube
        } else if lower.contains("music.apple") || lower.contains("apple music") {
            AppId::AppleMusic
        } else if lower.contains("twitch.tv") {
            AppId::Twitch
        } else if lower.contains("plex") {
            AppId::Plex
        } else if lower.contains("jellyfin") {
            AppId::Jellyfin
        } else if lower.contains("vesktop") || lower.contains("discord") {
            AppId::Discord
        } else if lower.contains("brave") || lower.contains("firefox") || lower.contains("chrome") {
            AppId::Browser
        } else {
            AppId::Unknown
        }
    }

    /// Get brand color for the app
    pub fn brand_color(&self) -> &'static str {
        match self {
            AppId::YouTube => "#ff0000",
            AppId::AppleMusic => "#fc3c44",
            AppId::Twitch => "#9146ff",
            AppId::Plex => "#e5a00d",
            AppId::Jellyfin => "#00a4dc",
            AppId::Discord => "#5865f2",
            AppId::Browser => "#ffffff",
            AppId::Unknown => "#ffffff",
        }
    }
}

/// Audio stream information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioStream {
    pub id: u32,
    pub app: AppId,
    pub name: String,
    pub volume: f32,     // 0.0 - 1.0+
    pub muted: bool,
    pub pid: Option<u32>,
}

/// Audio manager state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AudioState {
    pub streams: HashMap<u32, AudioStream>,
    pub default_sink: Option<String>,
    pub default_source: Option<String>,
}

impl AudioState {
    /// Get streams grouped by app
    pub fn by_app(&self) -> HashMap<AppId, Vec<&AudioStream>> {
        let mut map: HashMap<AppId, Vec<&AudioStream>> = HashMap::new();
        for stream in self.streams.values() {
            map.entry(stream.app).or_default().push(stream);
        }
        map
    }
}

// TODO: Implement actual PipeWire monitoring
// This will use pipewire-rs to:
// - Connect to PipeWire daemon
// - Monitor stream creation/destruction
// - Track volume changes
// - Provide volume control

/// Placeholder for PipeWire monitor
pub struct AudioMonitor;

impl AudioMonitor {
    pub fn new() -> Result<Self, AudioError> {
        // TODO: Initialize PipeWire connection
        Ok(Self)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AudioError {
    #[error("Failed to connect to PipeWire")]
    Connect,

    #[error("Stream not found: {0}")]
    StreamNotFound(u32),
}
