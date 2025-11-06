// Example: GTK4 window without traditional headerbar in Rust

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box, Button, Label, Orientation};
use libadwaita as adw;
use libadwaita::prelude::*;

fn main() {
    let app = adw::Application::builder()
        .application_id("com.example.NoHeaderbarDemo")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &adw::Application) {
    // Method 1: Completely borderless window (no decorations)
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(800)
        .default_height(600)
        .decorated(false)  // This removes ALL window decorations
        .build();

    // Method 2: Custom minimal header (if you want SOME controls)
    // Uncomment below and comment out .decorated(false) above
    /*
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(800)
        .default_height(600)
        .build();

    // Create custom minimal header
    let header = gtk4::HeaderBar::builder()
        .show_title_buttons(false)  // No min/max/close buttons
        .title_widget(&gtk4::Box::new(Orientation::Horizontal, 0))  // Empty title
        .build();

    window.set_titlebar(Some(&header));
    */

    // Method 3: Using Libadwaita's Window with custom content
    // This gives you more control over the window appearance
    /*
    let window = adw::Window::builder()
        .application(app)
        .default_width(800)
        .default_height(600)
        .build();

    // Remove the default header bar
    window.set_decorated(false);
    */

    // Create main content
    let main_box = Box::new(Orientation::Vertical, 0);

    // Add custom window controls if needed (for borderless window)
    if !window.is_decorated() {
        let control_bar = Box::new(Orientation::Horizontal, 6);
        control_bar.set_margin_top(6);
        control_bar.set_margin_start(12);
        control_bar.set_margin_end(12);

        // Add custom close button
        let close_btn = Button::with_label("×");
        close_btn.set_valign(gtk4::Align::Center);
        close_btn.connect_clicked(move |_| {
            std::process::exit(0);
        });

        control_bar.append(&Label::new(Some("Settings")));
        control_bar.set_hexpand(true);
        control_bar.append(&close_btn);

        main_box.append(&control_bar);
    }

    // Add main content area
    let content = Label::new(Some("Main Settings Content Area"));
    content.set_vexpand(true);
    main_box.append(&content);

    window.set_child(Some(&main_box));
    window.present();
}

// For Hyprland specifically, you can also use window rules to remove decorations:
// In hyprland.conf:
// windowrulev2 = noborder, class:^(settings-app)$
// windowrulev2 = noshadow, class:^(settings-app)$