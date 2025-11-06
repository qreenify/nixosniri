use gtk4::{prelude::*, *};
use std::cell::RefCell;
use std::rc::Rc;
use std::process::Command;
use crate::config::ConfigManager;

#[derive(Debug, Clone)]
struct Monitor {
    name: String,
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    scale: f64,
    primary: bool,
    connected: bool,
}

pub struct DisplayPage {
    config_manager: Rc<RefCell<ConfigManager>>,
}

impl DisplayPage {
    pub fn new(config_manager: Rc<RefCell<ConfigManager>>) -> Self {
        Self { config_manager }
    }

    pub fn build(&self) -> Box {
        let page_box = Box::new(Orientation::Vertical, 12);
        page_box.add_css_class("settings-page");

        let title = Label::new(Some("Display"));
        title.add_css_class("title-1");
        title.set_halign(Align::Start);
        title.set_margin_bottom(12);
        page_box.append(&title);

        // Monitor preview area (visual representation of monitors)
        page_box.append(&self.create_monitor_preview());

        // Display configuration section
        page_box.append(&self.create_display_config());

        // Display mode section
        page_box.append(&self.create_display_mode());

        // Color profile section
        page_box.append(&self.create_color_section());

        // Night light section
        page_box.append(&self.create_night_light());

        page_box
    }

    fn get_monitors(&self) -> Vec<Monitor> {
        let mut monitors = Vec::new();

        // Try to get monitor info from Hyprland
        if let Ok(output) = Command::new("hyprctl")
            .arg("monitors")
            .output()
        {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = output_str.lines().collect();

            let mut i = 0;
            while i < lines.len() {
                if lines[i].starts_with("Monitor ") {
                    let mut monitor = Monitor {
                        name: String::new(),
                        width: 1920,
                        height: 1080,
                        x: 0,
                        y: 0,
                        scale: 1.0,
                        primary: false,
                        connected: true,
                    };

                    // Parse monitor name
                    if let Some(name_part) = lines[i].split(' ').nth(1) {
                        monitor.name = name_part.to_string();
                    }

                    // Parse resolution and position (next line)
                    if i + 1 < lines.len() {
                        let res_line = lines[i + 1].trim();
                        // Parse format: "1920x1080@60.00000 at 300x-1080"
                        if let Some(at_split) = res_line.split(" at ").next() {
                            if let Some(res_part) = at_split.split('@').next() {
                                if let Some((w, h)) = res_part.split_once('x') {
                                    monitor.width = w.parse().unwrap_or(1920);
                                    monitor.height = h.parse().unwrap_or(1080);
                                }
                            }
                        }
                        if let Some(pos_part) = res_line.split(" at ").nth(1) {
                            if let Some((x, y)) = pos_part.split_once('x') {
                                monitor.x = x.parse().unwrap_or(0);
                                monitor.y = y.parse().unwrap_or(0);
                            }
                        }
                    }

                    // Check if primary (focused)
                    for j in i..std::cmp::min(i + 20, lines.len()) {
                        if lines[j].contains("focused: yes") {
                            monitor.primary = true;
                            break;
                        } else if lines[j].starts_with("Monitor ") {
                            break;
                        }
                    }

                    monitors.push(monitor);
                }
                i += 1;
            }
        }

        // Fallback if no monitors detected
        if monitors.is_empty() {
            monitors.push(Monitor {
                name: "Display 1".to_string(),
                width: 1920,
                height: 1080,
                x: 0,
                y: 0,
                scale: 1.0,
                primary: true,
                connected: true,
            });
        }

        monitors
    }

    fn create_monitor_preview(&self) -> Box {
        let section = self.create_section("Display Arrangement");

        // Get actual monitors
        let monitors = self.get_monitors();

        // Drawing area for monitor representation
        let preview_area = DrawingArea::new();
        preview_area.set_size_request(800, 400);
        preview_area.add_css_class("monitor-preview");
        preview_area.set_vexpand(false);

        // Draw monitors
        let monitors_clone = monitors.clone();
        preview_area.set_draw_func(move |_, cr, width, height| {
            // Background
            cr.set_source_rgba(0.08, 0.08, 0.08, 1.0);
            cr.rectangle(0.0, 0.0, width as f64, height as f64);
            cr.fill().unwrap();

            // Find the bounding box of all monitors
            let mut min_x = i32::MAX;
            let mut min_y = i32::MAX;
            let mut max_x = i32::MIN;
            let mut max_y = i32::MIN;

            for monitor in &monitors_clone {
                min_x = min_x.min(monitor.x);
                min_y = min_y.min(monitor.y);
                max_x = max_x.max(monitor.x + monitor.width);
                max_y = max_y.max(monitor.y + monitor.height);
            }

            let total_width = (max_x - min_x) as f64;
            let total_height = (max_y - min_y) as f64;

            // Calculate scale to fit in preview area with padding
            let padding = 40.0;
            let scale = ((width as f64 - 2.0 * padding) / total_width)
                .min((height as f64 - 2.0 * padding) / total_height)
                .min(0.15); // Max scale to keep monitors reasonably sized

            // Calculate offset to center the monitors
            let offset_x = (width as f64 - total_width * scale) / 2.0;
            let offset_y = (height as f64 - total_height * scale) / 2.0;

            // Draw each monitor
            for (idx, monitor) in monitors_clone.iter().enumerate() {
                let x = offset_x + (monitor.x - min_x) as f64 * scale;
                let y = offset_y + (monitor.y - min_y) as f64 * scale;
                let w = monitor.width as f64 * scale;
                let h = monitor.height as f64 * scale;

                // Monitor bezel/frame
                if monitor.primary {
                    cr.set_source_rgba(0.3, 0.5, 0.7, 1.0); // Blue for primary
                } else {
                    cr.set_source_rgba(0.25, 0.25, 0.25, 1.0); // Gray for others
                }
                cr.rectangle(x, y, w, h);
                cr.fill().unwrap();

                // Monitor screen
                cr.set_source_rgba(0.12, 0.12, 0.12, 1.0);
                let inset = 3.0;
                cr.rectangle(x + inset, y + inset, w - 2.0 * inset, h - 2.0 * inset);
                cr.fill().unwrap();

                // Draw monitor info
                cr.set_source_rgba(1.0, 1.0, 1.0, 0.9);
                cr.set_font_size(14.0);

                // Monitor name
                let text = &monitor.name;
                let extents = cr.text_extents(text).unwrap();
                cr.move_to(
                    x + (w - extents.width()) / 2.0,
                    y + h / 2.0 - 10.0,
                );
                cr.show_text(text).unwrap();

                // Resolution
                let resolution = format!("{}×{}", monitor.width, monitor.height);
                let extents = cr.text_extents(&resolution).unwrap();
                cr.set_font_size(11.0);
                cr.move_to(
                    x + (w - extents.width()) / 2.0,
                    y + h / 2.0 + 10.0,
                );
                cr.show_text(&resolution).unwrap();

                // Number badge
                cr.set_source_rgba(0.8, 0.8, 0.8, 0.9);
                cr.arc(x + 15.0, y + 15.0, 10.0, 0.0, 2.0 * std::f64::consts::PI);
                cr.fill().unwrap();

                cr.set_source_rgba(0.0, 0.0, 0.0, 1.0);
                cr.set_font_size(12.0);
                let num = format!("{}", idx + 1);
                cr.move_to(x + 11.0, y + 19.0);
                cr.show_text(&num).unwrap();
            }

            // Draw grid dots for alignment reference
            cr.set_source_rgba(0.3, 0.3, 0.3, 0.3);
            let grid_spacing = 20.0;
            let mut gx = padding;
            while gx < width as f64 - padding {
                let mut gy = padding;
                while gy < height as f64 - padding {
                    cr.arc(gx, gy, 1.0, 0.0, 2.0 * std::f64::consts::PI);
                    cr.fill().unwrap();
                    gy += grid_spacing;
                }
                gx += grid_spacing;
            }
        });

        section.append(&preview_area);

        // Arrangement controls
        let controls = Box::new(Orientation::Horizontal, 8);
        controls.set_halign(Align::Center);
        controls.set_margin_top(8);

        let identify_btn = Button::with_label("Identify Displays");
        identify_btn.add_css_class("suggested-action");
        identify_btn.connect_clicked(move |_| {
            // Would trigger display identification
            println!("Identify displays clicked");
        });
        controls.append(&identify_btn);

        let mirror_check = CheckButton::with_label("Mirror displays");
        controls.append(&mirror_check);

        let apply_btn = Button::with_label("Apply");
        apply_btn.set_sensitive(false); // Enable when changes are made
        controls.append(&apply_btn);

        section.append(&controls);

        // Monitor info
        let info_box = Box::new(Orientation::Horizontal, 12);
        info_box.set_halign(Align::Center);
        info_box.set_margin_top(8);

        for monitor in &monitors {
            let chip = Box::new(Orientation::Horizontal, 4);
            chip.add_css_class("monitor-chip");

            let indicator = DrawingArea::new();
            indicator.set_size_request(12, 12);
            let is_primary = monitor.primary;
            indicator.set_draw_func(move |_, cr, _, _| {
                if is_primary {
                    cr.set_source_rgba(0.3, 0.5, 0.7, 1.0);
                } else {
                    cr.set_source_rgba(0.3, 0.3, 0.3, 1.0);
                }
                cr.arc(6.0, 6.0, 5.0, 0.0, 2.0 * std::f64::consts::PI);
                cr.fill().unwrap();
            });
            chip.append(&indicator);

            let label = Label::new(Some(&format!("{}: {}×{}",
                monitor.name, monitor.width, monitor.height)));
            label.add_css_class("caption");
            chip.append(&label);

            info_box.append(&chip);
        }

        section.append(&info_box);

        section
    }

    fn create_display_config(&self) -> Box {
        let section = self.create_section("Display Configuration");

        let monitors = self.get_monitors();
        let monitor_names: Vec<String> = monitors.iter()
            .map(|m| {
                if m.primary {
                    format!("{} - Primary", m.name)
                } else {
                    m.name.clone()
                }
            })
            .collect();

        let monitor_name_refs: Vec<&str> = monitor_names.iter().map(|s| s.as_str()).collect();

        // Display selector
        let display_row = self.create_settings_row(
            "Display",
            Some("Select display to configure"),
        );

        let display_dropdown = DropDown::from_strings(&monitor_name_refs);
        display_dropdown.set_selected(0);
        display_row.append(&display_dropdown);
        section.append(&display_row);

        // Enable/disable toggle
        let enable_row = self.create_settings_row(
            "Enable this display",
            Some("Turn this display on or off"),
        );
        let enable_switch = Switch::new();
        enable_switch.set_active(true);
        enable_switch.set_valign(Align::Center);
        enable_row.append(&enable_switch);
        section.append(&enable_row);

        // Primary display toggle
        let primary_row = self.create_settings_row(
            "Primary display",
            Some("Make this your main display"),
        );
        let primary_switch = Switch::new();
        primary_switch.set_active(monitors.get(0).map_or(false, |m| m.primary));
        primary_switch.set_valign(Align::Center);
        primary_row.append(&primary_switch);
        section.append(&primary_row);

        // Resolution
        let resolution_row = self.create_settings_row(
            "Resolution",
            Some("Display resolution"),
        );
        let resolution_dropdown = DropDown::from_strings(&[
            "3840 × 2160",
            "2560 × 1440",
            "1920 × 1080",
            "1680 × 1050",
            "1440 × 900",
            "1366 × 768",
            "1280 × 720",
        ]);

        // Set current resolution if found
        if let Some(monitor) = monitors.first() {
            let current_res = format!("{} × {}", monitor.width, monitor.height);
            let resolutions = vec![
                "3840 × 2160", "2560 × 1440", "1920 × 1080",
                "1680 × 1050", "1440 × 900", "1366 × 768", "1280 × 720"
            ];
            if let Some(pos) = resolutions.iter().position(|&r| r == current_res) {
                resolution_dropdown.set_selected(pos as u32);
            }
        }

        resolution_row.append(&resolution_dropdown);
        section.append(&resolution_row);

        // Refresh rate
        let refresh_row = self.create_settings_row(
            "Refresh Rate",
            Some("Higher rates provide smoother motion"),
        );
        let refresh_dropdown = DropDown::from_strings(&[
            "60 Hz",
            "75 Hz",
            "100 Hz",
            "120 Hz",
            "144 Hz",
            "155 Hz",
            "165 Hz",
            "240 Hz",
        ]);
        refresh_dropdown.set_selected(0);
        refresh_row.append(&refresh_dropdown);
        section.append(&refresh_row);

        // Orientation
        let orientation_row = self.create_settings_row(
            "Orientation",
            Some("Rotate the display"),
        );
        let orientation_dropdown = DropDown::from_strings(&[
            "Standard",
            "90° Right",
            "180° Inverted",
            "90° Left",
        ]);
        orientation_dropdown.set_selected(0);
        orientation_row.append(&orientation_dropdown);
        section.append(&orientation_row);

        // Scale
        let scale_row = self.create_settings_row(
            "Scale",
            Some("Make everything larger or smaller"),
        );

        let scale_box = Box::new(Orientation::Horizontal, 8);
        let scale_slider = Scale::with_range(Orientation::Horizontal, 100.0, 200.0, 25.0);
        scale_slider.set_value(100.0);
        scale_slider.set_hexpand(true);
        scale_slider.set_draw_value(true);
        scale_slider.set_value_pos(PositionType::Right);

        // Add marks for common values
        scale_slider.add_mark(100.0, PositionType::Bottom, Some("100%"));
        scale_slider.add_mark(125.0, PositionType::Bottom, Some("125%"));
        scale_slider.add_mark(150.0, PositionType::Bottom, Some("150%"));
        scale_slider.add_mark(175.0, PositionType::Bottom, Some("175%"));
        scale_slider.add_mark(200.0, PositionType::Bottom, Some("200%"));

        scale_box.append(&scale_slider);
        scale_row.append(&scale_box);
        section.append(&scale_row);

        section
    }

    fn create_display_mode(&self) -> Box {
        let section = self.create_section("Display Mode");

        // Variable refresh rate (VRR/FreeSync/G-Sync)
        let vrr_row = self.create_settings_row(
            "Variable Refresh Rate",
            Some("Enable adaptive sync (FreeSync/G-Sync)"),
        );
        let vrr_switch = Switch::new();
        vrr_switch.set_valign(Align::Center);
        vrr_row.append(&vrr_switch);
        section.append(&vrr_row);

        // HDR
        let hdr_row = self.create_settings_row(
            "High Dynamic Range (HDR)",
            Some("Enable HDR if your display supports it"),
        );
        let hdr_switch = Switch::new();
        hdr_switch.set_valign(Align::Center);
        hdr_row.append(&hdr_switch);
        section.append(&hdr_row);

        // Fractional scaling
        let fractional_row = self.create_settings_row(
            "Fractional Scaling",
            Some("Allow scaling in 25% increments (experimental)"),
        );
        let fractional_switch = Switch::new();
        fractional_switch.set_valign(Align::Center);
        fractional_row.append(&fractional_switch);
        section.append(&fractional_row);

        section
    }

    fn create_color_section(&self) -> Box {
        let section = self.create_section("Color");

        // Color profile
        let profile_row = self.create_settings_row(
            "Color Profile",
            Some("Select a color profile for this display"),
        );
        let profile_dropdown = DropDown::from_strings(&[
            "Standard (sRGB)",
            "Adobe RGB",
            "DCI-P3",
            "Display P3",
            "Rec. 2020",
            "Custom ICC Profile...",
        ]);
        profile_dropdown.set_selected(0);
        profile_row.append(&profile_dropdown);
        section.append(&profile_row);

        // Brightness (if supported)
        let brightness_row = self.create_settings_row(
            "Brightness",
            Some("Adjust display brightness"),
        );

        let brightness_scale = Scale::with_range(Orientation::Horizontal, 0.0, 100.0, 5.0);
        brightness_scale.set_value(100.0);
        brightness_scale.set_hexpand(true);
        brightness_scale.set_draw_value(true);
        brightness_row.append(&brightness_scale);
        section.append(&brightness_row);

        section
    }

    fn create_night_light(&self) -> Box {
        let section = self.create_section("Night Light");

        // Night light toggle
        let night_light_row = self.create_settings_row(
            "Night Light",
            Some("Reduces blue light to help you sleep better"),
        );
        let night_light_switch = Switch::new();
        night_light_switch.set_valign(Align::Center);
        night_light_row.append(&night_light_switch);
        section.append(&night_light_row);

        // Schedule
        let schedule_row = self.create_settings_row(
            "Schedule",
            Some("When to enable night light"),
        );
        let schedule_dropdown = DropDown::from_strings(&[
            "Sunset to Sunrise",
            "Manual Schedule",
            "Always On",
        ]);
        schedule_dropdown.set_selected(0);
        schedule_row.append(&schedule_dropdown);
        section.append(&schedule_row);

        // Color temperature
        let temp_row = self.create_settings_row(
            "Color Temperature",
            Some("Warmer colors are easier on the eyes"),
        );

        let temp_box = Box::new(Orientation::Horizontal, 8);

        let cool_label = Label::new(Some("Cool"));
        cool_label.add_css_class("dim-label");
        temp_box.append(&cool_label);

        let temp_scale = Scale::with_range(Orientation::Horizontal, 3000.0, 6500.0, 100.0);
        temp_scale.set_value(4500.0);
        temp_scale.set_hexpand(true);
        temp_scale.set_draw_value(false);
        temp_box.append(&temp_scale);

        let warm_label = Label::new(Some("Warm"));
        warm_label.add_css_class("dim-label");
        temp_box.append(&warm_label);

        temp_row.append(&temp_box);
        section.append(&temp_row);

        section
    }

    fn create_section(&self, title: &str) -> Box {
        let section = Box::new(Orientation::Vertical, 8);
        section.add_css_class("settings-group");

        let title_label = Label::new(Some(title));
        title_label.add_css_class("settings-group-title");
        title_label.set_halign(Align::Start);
        section.append(&title_label);

        section
    }

    fn create_settings_row(&self, title: &str, subtitle: Option<&str>) -> Box {
        let row = Box::new(Orientation::Horizontal, 12);
        row.add_css_class("settings-row");

        let text_box = Box::new(Orientation::Vertical, 2);
        text_box.set_hexpand(true);

        let title_label = Label::new(Some(title));
        title_label.add_css_class("settings-row-title");
        title_label.set_halign(Align::Start);
        text_box.append(&title_label);

        if let Some(subtitle) = subtitle {
            let subtitle_label = Label::new(Some(subtitle));
            subtitle_label.add_css_class("settings-row-subtitle");
            subtitle_label.set_halign(Align::Start);
            text_box.append(&subtitle_label);
        }

        row.append(&text_box);
        row
    }
}