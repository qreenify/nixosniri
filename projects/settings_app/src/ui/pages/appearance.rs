use gtk4::{prelude::*, *};
use std::cell::RefCell;
use std::rc::Rc;
use crate::config::ConfigManager;

pub struct AppearancePage {
    config_manager: Rc<RefCell<ConfigManager>>,
}

impl AppearancePage {
    pub fn new(config_manager: Rc<RefCell<ConfigManager>>) -> Self {
        Self { config_manager }
    }

    pub fn build(&self) -> Box {
        let page_box = Box::new(Orientation::Vertical, 12);
        page_box.add_css_class("settings-page");

        // Page title
        let title = Label::new(Some("Appearance"));
        title.add_css_class("title-1");
        title.set_halign(Align::Start);
        title.set_margin_bottom(12);
        page_box.append(&title);

        // Empty page - you can add your own settings here
        let placeholder = Label::new(Some("Add your appearance settings here"));
        placeholder.add_css_class("dim-label");
        placeholder.set_halign(Align::Center);
        placeholder.set_valign(Align::Center);
        placeholder.set_vexpand(true);
        page_box.append(&placeholder);

        page_box
    }
}