use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use anyhow::{Result, anyhow};

/// Hyprland IPC client for sending commands and getting responses
pub struct HyprlandIPC {
    socket_path: PathBuf,
}

impl HyprlandIPC {
    pub fn new() -> Result<Self> {
        let instance = super::get_hyprland_instance()
            .ok_or_else(|| anyhow!("Hyprland is not running"))?;

        let socket_path = PathBuf::from(format!("/tmp/hypr/{}/.socket.sock", instance));

        if !socket_path.exists() {
            return Err(anyhow!("Hyprland socket not found"));
        }

        Ok(Self { socket_path })
    }

    /// Send a command to Hyprland via IPC
    pub fn dispatch(&self, command: &str) -> Result<String> {
        let mut stream = UnixStream::connect(&self.socket_path)?;

        // Send the command
        stream.write_all(command.as_bytes())?;
        stream.flush()?;

        // Read the response
        let mut response = String::new();
        stream.read_to_string(&mut response)?;

        Ok(response)
    }

    /// Reload Hyprland configuration
    pub fn reload_config(&self) -> Result<()> {
        self.dispatch("reload")?;
        Ok(())
    }

    /// Get current workspace
    pub fn get_active_workspace(&self) -> Result<i32> {
        let response = self.dispatch("j/activeworkspace")?;
        // Parse JSON response
        // Simplified - real implementation would use serde_json
        Ok(1)
    }

    /// Set a config option
    pub fn set_option(&self, option: &str, value: &str) -> Result<()> {
        let command = format!("keyword {} {}", option, value);
        self.dispatch(&command)?;
        Ok(())
    }

    /// Get monitors information
    pub fn get_monitors(&self) -> Result<String> {
        self.dispatch("j/monitors")
    }

    /// Get all keybindings
    pub fn get_binds(&self) -> Result<String> {
        self.dispatch("j/binds")
    }
}