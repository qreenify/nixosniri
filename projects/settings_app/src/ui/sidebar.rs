use gtk4::{prelude::*, *};

pub struct Sidebar;

impl Sidebar {
    pub fn new() -> Self {
        Self
    }

    pub fn build_with_navigation<F>(&self, on_item_selected: F) -> Box
    where
        F: Fn(&str) + Clone + 'static,
    {
        let sidebar_box = Box::new(Orientation::Vertical, 4);
        sidebar_box.add_css_class("sidebar");
        sidebar_box.set_size_request(220, -1);

        // Add search entry at the top
        let search_entry = SearchEntry::new();
        search_entry.set_placeholder_text(Some("Search settings..."));
        search_entry.set_margin_bottom(8);
        sidebar_box.append(&search_entry);

        // Create scrolled window for items
        let scrolled = ScrolledWindow::builder()
            .hscrollbar_policy(PolicyType::Never)
            .vscrollbar_policy(PolicyType::Automatic)
            .vexpand(true)
            .build();

        let items_box = Box::new(Orientation::Vertical, 2);

        // Define sidebar items
        let items = vec![
            ("appearance", "Appearance", "preferences-desktop-wallpaper-symbolic"),
            ("display", "Display", "video-display-symbolic"),
            ("input", "Input Devices", "input-keyboard-symbolic"),
            ("windows", "Windows", "window-symbolic"),
            ("keybindings", "Keybindings", "key-symbolic"),
            ("applications", "Applications", "application-x-executable-symbolic"),
            ("about", "About", "help-about-symbolic"),
        ];

        // Add all sidebar items
        for (id, label, icon) in items {
            let item_widget = self.create_sidebar_item(id, label, icon, &on_item_selected);
            items_box.append(&item_widget);
        }

        scrolled.set_child(Some(&items_box));
        sidebar_box.append(&scrolled);

        sidebar_box
    }

    fn create_sidebar_item<F>(&self, id: &str, label: &str, icon: &str, callback: &F) -> Box
    where
        F: Fn(&str) + Clone + 'static,
    {
        let item_box = Box::new(Orientation::Horizontal, 0);
        item_box.add_css_class("sidebar-item");

        // Create clickable button
        let button = Button::new();
        button.add_css_class("flat");
        button.set_hexpand(true);

        let content_box = Box::new(Orientation::Horizontal, 12);
        content_box.set_margin_start(4);
        content_box.set_margin_end(4);
        content_box.set_margin_top(8);
        content_box.set_margin_bottom(8);

        // Icon
        let icon_widget = Image::from_icon_name(icon);
        icon_widget.set_pixel_size(20);
        icon_widget.add_css_class("sidebar-icon");
        content_box.append(&icon_widget);

        // Label
        let label_widget = Label::new(Some(label));
        label_widget.set_halign(Align::Start);
        label_widget.add_css_class("sidebar-label");
        content_box.append(&label_widget);

        button.set_child(Some(&content_box));

        // Connect click handler
        let id_clone = id.to_string();
        let callback_clone = callback.clone();
        button.connect_clicked(move |_| {
            callback_clone(&id_clone);
        });

        item_box.append(&button);
        item_box
    }
}