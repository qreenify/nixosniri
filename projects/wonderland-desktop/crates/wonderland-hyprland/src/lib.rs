//! Hyprland IPC communication
//!
//! Provides typed access to Hyprland socket commands.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;

/// Hyprland client for IPC communication
pub struct HyprlandClient {
    socket_path: PathBuf,
}

impl HyprlandClient {
    /// Create a new client, auto-detecting socket path
    pub fn new() -> Result<Self, HyprlandError> {
        let his = std::env::var("HYPRLAND_INSTANCE_SIGNATURE")
            .map_err(|_| HyprlandError::NotRunning)?;

        let xdg_runtime = std::env::var("XDG_RUNTIME_DIR")
            .unwrap_or_else(|_| "/run/user/1000".to_string());

        let socket_path = PathBuf::from(xdg_runtime)
            .join("hypr")
            .join(&his)
            .join(".socket.sock");

        Ok(Self { socket_path })
    }

    /// Send a command and get JSON response
    pub async fn command(&self, cmd: &str) -> Result<String, HyprlandError> {
        let mut stream = UnixStream::connect(&self.socket_path)
            .await
            .map_err(|e| HyprlandError::Connect(e))?;

        stream
            .write_all(format!("j/{}", cmd).as_bytes())
            .await
            .map_err(|e| HyprlandError::Write(e))?;

        let mut response = String::new();
        stream
            .read_to_string(&mut response)
            .await
            .map_err(|e| HyprlandError::Read(e))?;

        Ok(response)
    }

    /// Get active window info
    pub async fn active_window(&self) -> Result<Window, HyprlandError> {
        let response = self.command("activewindow").await?;
        serde_json::from_str(&response).map_err(HyprlandError::Parse)
    }

    /// Get all workspaces
    pub async fn workspaces(&self) -> Result<Vec<Workspace>, HyprlandError> {
        let response = self.command("workspaces").await?;
        serde_json::from_str(&response).map_err(HyprlandError::Parse)
    }

    /// Get all monitors
    pub async fn monitors(&self) -> Result<Vec<Monitor>, HyprlandError> {
        let response = self.command("monitors").await?;
        serde_json::from_str(&response).map_err(HyprlandError::Parse)
    }

    /// Dispatch a Hyprland command
    pub async fn dispatch(&self, args: &str) -> Result<(), HyprlandError> {
        let mut stream = UnixStream::connect(&self.socket_path)
            .await
            .map_err(|e| HyprlandError::Connect(e))?;

        stream
            .write_all(format!("/dispatch {}", args).as_bytes())
            .await
            .map_err(|e| HyprlandError::Write(e))?;

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Window {
    pub address: String,
    pub title: String,
    pub class: String,
    pub pid: i32,
    pub workspace: WorkspaceRef,
    pub floating: bool,
    pub fullscreen: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceRef {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: i32,
    pub name: String,
    pub monitor: String,
    pub windows: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Monitor {
    pub id: i32,
    pub name: String,
    pub width: i32,
    pub height: i32,
    pub x: i32,
    pub y: i32,
    pub scale: f32,
    #[serde(rename = "activeWorkspace")]
    pub active_workspace: WorkspaceRef,
}

#[derive(Debug, thiserror::Error)]
pub enum HyprlandError {
    #[error("Hyprland is not running")]
    NotRunning,

    #[error("Failed to connect to Hyprland socket: {0}")]
    Connect(std::io::Error),

    #[error("Failed to write to socket: {0}")]
    Write(std::io::Error),

    #[error("Failed to read from socket: {0}")]
    Read(std::io::Error),

    #[error("Failed to parse response: {0}")]
    Parse(serde_json::Error),
}
