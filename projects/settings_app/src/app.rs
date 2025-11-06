use gtk4::{prelude::*, *};
use libadwaita as adw;
use libadwaita::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use crate::ui::{sidebar::Sidebar, pages::*};
use crate::config::ConfigManager;

pub struct HyprlandSettings {
    app: adw::Application,
    window: ApplicationWindow,
    config_manager: Rc<RefCell<ConfigManager>>,
    current_page: Rc<RefCell<String>>,
}

impl HyprlandSettings {
    pub fn new(app: &adw::Application) -> Self {
        // Create the main window without decorations
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Hyprland Settings")
            .default_width(980)
            .default_height(640)
            .decorated(false)  // No title bar
            .build();

        // Apply custom CSS for styling
        Self::load_css();

        Self {
            app: app.clone(),
            window,
            config_manager: Rc::new(RefCell::new(ConfigManager::new())),
            current_page: Rc::new(RefCell::new("appearance".to_string())),
        }
    }

    pub fn build_ui(&self) {
        // Create main container
        let main_box = Box::new(Orientation::Vertical, 0);
        main_box.add_css_class("main-window");

        // Create horizontal split for sidebar and content (NO HEADER)
        let content_box = Box::new(Orientation::Horizontal, 0);
        content_box.add_css_class("content-area");

        // Create sidebar (will be created later with navigation)

        // Create stack for different settings pages
        let stack = Stack::new();
        stack.add_css_class("settings-stack");
        stack.set_hexpand(true);
        stack.set_vexpand(true);
        stack.set_transition_type(StackTransitionType::Crossfade);
        stack.set_transition_duration(200);

        // Add all settings pages
        self.setup_pages(&stack);

        // Wrap stack in scrolled window
        let scrolled = ScrolledWindow::builder()
            .hscrollbar_policy(PolicyType::Never)
            .vscrollbar_policy(PolicyType::Automatic)
            .child(&stack)
            .hexpand(true)
            .vexpand(true)
            .build();
        scrolled.add_css_class("content-scroll");

        // Create sidebar with navigation
        let sidebar = Sidebar::new();
        let stack_clone = stack.clone();
        let sidebar_widget = sidebar.build_with_navigation(move |page_name| {
            stack_clone.set_visible_child_name(page_name);
        });

        // Insert sidebar at the beginning of content_box
        content_box.prepend(&sidebar_widget);
        content_box.append(&scrolled);

        // Add content directly to window (no header)
        main_box.append(&content_box);

        // Set initial page
        stack.set_visible_child_name("appearance");

        // Add the main box to window
        self.window.set_child(Some(&main_box));

        // Show everything
        self.window.present();
    }

    fn create_header(&self) -> Box {
        let header = Box::new(Orientation::Horizontal, 0);
        header.add_css_class("custom-header");
        header.set_size_request(-1, 40);

        // Left side - App title
        let title_box = Box::new(Orientation::Horizontal, 8);
        title_box.set_margin_start(12);

        let icon = Image::from_icon_name("preferences-system");
        icon.set_pixel_size(20);
        title_box.append(&icon);

        let title = Label::new(Some("Hyprland Settings"));
        title.add_css_class("title-label");
        title_box.append(&title);

        header.append(&title_box);

        // Center spacer
        let spacer = Box::new(Orientation::Horizontal, 0);
        spacer.set_hexpand(true);
        header.append(&spacer);

        // Right side - Window controls
        let controls_box = Box::new(Orientation::Horizontal, 4);
        controls_box.set_margin_end(8);
        controls_box.add_css_class("window-controls");

        // Minimize button
        let minimize_btn = Button::new();
        minimize_btn.set_icon_name("window-minimize-symbolic");
        minimize_btn.add_css_class("header-button");
        minimize_btn.add_css_class("minimize");

        // Maximize button
        let maximize_btn = Button::new();
        maximize_btn.set_icon_name("window-maximize-symbolic");
        maximize_btn.add_css_class("header-button");
        maximize_btn.add_css_class("maximize");

        // Close button
        let close_btn = Button::new();
        close_btn.set_icon_name("window-close-symbolic");
        close_btn.add_css_class("header-button");
        close_btn.add_css_class("close");

        controls_box.append(&minimize_btn);
        controls_box.append(&maximize_btn);
        controls_box.append(&close_btn);
        header.append(&controls_box);

        // Connect button signals
        let window_clone = self.window.clone();
        minimize_btn.connect_clicked(move |_| {
            window_clone.minimize();
        });

        let window_clone = self.window.clone();
        maximize_btn.connect_clicked(move |_| {
            if window_clone.is_maximized() {
                window_clone.unmaximize();
            } else {
                window_clone.maximize();
            }
        });

        let window_clone = self.window.clone();
        close_btn.connect_clicked(move |_| {
            window_clone.close();
        });

        header
    }


    fn setup_pages(&self, stack: &Stack) {
        // Add appearance page
        let appearance_page = appearance::AppearancePage::new(self.config_manager.clone());
        stack.add_named(&appearance_page.build(), Some("appearance"));

        // Add display page
        let display_page = display::DisplayPage::new(self.config_manager.clone());
        stack.add_named(&display_page.build(), Some("display"));

        // Add input page
        let input_page = input::InputPage::new(self.config_manager.clone());
        stack.add_named(&input_page.build(), Some("input"));

        // Add windows page
        let windows_page = windows::WindowsPage::new(self.config_manager.clone());
        stack.add_named(&windows_page.build(), Some("windows"));

        // Add keybindings page
        let keybindings_page = keybindings::KeybindingsPage::new(self.config_manager.clone());
        stack.add_named(&keybindings_page.build(), Some("keybindings"));

        // Add applications page
        let apps_page = applications::ApplicationsPage::new(self.config_manager.clone());
        stack.add_named(&apps_page.build(), Some("applications"));

        // Add about page
        let about_page = about::AboutPage::new();
        stack.add_named(&about_page.build(), Some("about"));
    }


    fn setup_window_controls(&self, header: &Box) {
        // Allow dragging the window by the header
        let drag_controller = GestureDrag::new();

        let window_clone = self.window.clone();
        drag_controller.connect_drag_begin(move |_, _x, _y| {
            // Start window drag
            // Hyprland will handle window movement with Alt+drag
            // or we could implement custom dragging here
            _ = &window_clone; // Suppress unused warning
        });

        header.add_controller(drag_controller);
    }

    fn load_css() {
        let css_provider = CssProvider::new();
        css_provider.load_from_string(include_str!("styles.css"));

        gtk4::style_context_add_provider_for_display(
            &gdk::Display::default().expect("Could not get default display"),
            &css_provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}