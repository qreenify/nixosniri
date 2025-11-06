use gtk4::{prelude::*, *};

pub struct AboutPage;

impl AboutPage {
    pub fn new() -> Self {
        Self
    }

    pub fn build(&self) -> Box {
        let page_box = Box::new(Orientation::Vertical, 12);
        page_box.add_css_class("settings-page");

        let title = Label::new(Some("About"));
        title.add_css_class("title-1");
        title.set_halign(Align::Start);
        page_box.append(&title);

        // App info
        let info_group = Box::new(Orientation::Vertical, 8);
        info_group.add_css_class("settings-group");

        let app_name = Label::new(Some("Hyprland Settings"));
        app_name.add_css_class("title-2");
        app_name.set_halign(Align::Center);
        info_group.append(&app_name);

        let version = Label::new(Some("Version 0.1.0"));
        version.set_halign(Align::Center);
        info_group.append(&version);

        let description = Label::new(Some("A modern settings application for Hyprland"));
        description.set_halign(Align::Center);
        description.set_wrap(true);
        info_group.append(&description);

        page_box.append(&info_group);

        // System info
        let system_group = Box::new(Orientation::Vertical, 8);
        system_group.add_css_class("settings-group");

        let system_title = Label::new(Some("System Information"));
        system_title.add_css_class("settings-group-title");
        system_title.set_halign(Align::Start);
        system_group.append(&system_title);

        // Get system info
        let os_info = format!("OS: {}", std::env::consts::OS);
        let arch_info = format!("Architecture: {}", std::env::consts::ARCH);

        let os_label = Label::new(Some(&os_info));
        os_label.set_halign(Align::Start);
        system_group.append(&os_label);

        let arch_label = Label::new(Some(&arch_info));
        arch_label.set_halign(Align::Start);
        system_group.append(&arch_label);

        page_box.append(&system_group);

        page_box
    }
}