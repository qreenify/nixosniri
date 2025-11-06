use gtk4::{prelude::*, *};

pub struct Header;

impl Header {
    pub fn new() -> Self {
        Self
    }

    pub fn build(&self) -> Box {
        let header_box = Box::new(Orientation::Horizontal, 0);
        header_box.add_css_class("custom-header");
        header_box.set_size_request(-1, 40);

        // This is created in app.rs for now
        // We can refactor this later if needed

        header_box
    }
}