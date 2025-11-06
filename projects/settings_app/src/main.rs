mod app;
mod config;
mod ui;
mod utils;

use libadwaita as adw;
use libadwaita::prelude::*;

fn main() {
    // Initialize GTK and Adwaita
    gtk4::init().expect("Failed to initialize GTK");

    // Create the application
    let app = adw::Application::builder()
        .application_id("com.hyprland.Settings")
        .build();

    // Connect the activate signal
    app.connect_activate(move |app| {
        crate::app::HyprlandSettings::new(app).build_ui();
    });

    // Run the application
    let args: Vec<String> = std::env::args().collect();
    app.run_with_args(&args);
}