pub mod ipc;

use std::process::Command;
use anyhow::Result;

/// Execute a shell command and return the output
pub fn run_command(cmd: &str, args: &[&str]) -> Result<String> {
    let output = Command::new(cmd)
        .args(args)
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Get the current Hyprland instance signature
pub fn get_hyprland_instance() -> Option<String> {
    std::env::var("HYPRLAND_INSTANCE_SIGNATURE").ok()
}

/// Check if Hyprland is running
pub fn is_hyprland_running() -> bool {
    get_hyprland_instance().is_some()
}