//! Iced theme implementation

use crate::loader::Theme;
use iced::widget::{button, container, scrollable, text, text_input};
use iced::{Background, Border, Color};

/// Iced-compatible theme wrapper
#[derive(Debug, Clone)]
pub struct WonderlandTheme {
    pub theme: Theme,
}

impl WonderlandTheme {
    pub fn new(theme: Theme) -> Self {
        Self { theme }
    }

    /// Get foreground as Iced color
    pub fn foreground(&self) -> Color {
        self.theme.foreground.to_iced()
    }

    /// Get background as Iced color
    pub fn background(&self) -> Color {
        self.theme.background.to_iced()
    }

    /// Get primary accent as Iced color
    pub fn primary(&self) -> Color {
        self.theme.primary.to_iced()
    }

    /// Get surface color (slightly different from background)
    pub fn surface(&self) -> Color {
        self.theme.surface.to_iced()
    }
}

// Container styling
impl container::Catalog for WonderlandTheme {
    type Class<'a> = ContainerClass;

    fn default<'a>() -> Self::Class<'a> {
        ContainerClass::Default
    }

    fn style(&self, class: &Self::Class<'_>) -> container::Style {
        match class {
            ContainerClass::Default => container::Style {
                background: Some(Background::Color(self.background())),
                text_color: Some(self.foreground()),
                ..Default::default()
            },
            ContainerClass::Surface => container::Style {
                background: Some(Background::Color(self.surface())),
                text_color: Some(self.foreground()),
                border: Border {
                    color: self.theme.border.to_iced(),
                    width: 1.0,
                    radius: 8.0.into(),
                },
                ..Default::default()
            },
            ContainerClass::Transparent => container::Style::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ContainerClass {
    #[default]
    Default,
    Surface,
    Transparent,
}

// Button styling
impl button::Catalog for WonderlandTheme {
    type Class<'a> = ButtonClass;

    fn default<'a>() -> Self::Class<'a> {
        ButtonClass::Primary
    }

    fn style(&self, class: &Self::Class<'_>, status: button::Status) -> button::Style {
        let base = match class {
            ButtonClass::Primary => button::Style {
                background: Some(Background::Color(self.primary())),
                text_color: self.background(),
                border: Border {
                    radius: 6.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            },
            ButtonClass::Secondary => button::Style {
                background: Some(Background::Color(self.surface())),
                text_color: self.foreground(),
                border: Border {
                    color: self.theme.border.to_iced(),
                    width: 1.0,
                    radius: 6.0.into(),
                },
                ..Default::default()
            },
            ButtonClass::Text => button::Style {
                background: None,
                text_color: self.primary(),
                ..Default::default()
            },
        };

        match status {
            button::Status::Active => base,
            button::Status::Hovered => button::Style {
                background: base.background.map(|bg| {
                    if let Background::Color(c) = bg {
                        Background::Color(Color {
                            a: c.a * 0.9,
                            ..c
                        })
                    } else {
                        bg
                    }
                }),
                ..base
            },
            button::Status::Pressed => button::Style {
                background: base.background.map(|bg| {
                    if let Background::Color(c) = bg {
                        Background::Color(Color {
                            a: c.a * 0.7,
                            ..c
                        })
                    } else {
                        bg
                    }
                }),
                ..base
            },
            button::Status::Disabled => button::Style {
                background: base.background.map(|bg| {
                    if let Background::Color(c) = bg {
                        Background::Color(Color {
                            a: c.a * 0.5,
                            ..c
                        })
                    } else {
                        bg
                    }
                }),
                text_color: Color {
                    a: base.text_color.a * 0.5,
                    ..base.text_color
                },
                ..base
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ButtonClass {
    #[default]
    Primary,
    Secondary,
    Text,
}

// Text styling
impl text::Catalog for WonderlandTheme {
    type Class<'a> = TextClass;

    fn default<'a>() -> Self::Class<'a> {
        TextClass::Default
    }

    fn style(&self, class: &Self::Class<'_>) -> text::Style {
        match class {
            TextClass::Default => text::Style {
                color: Some(self.foreground()),
            },
            TextClass::Muted => text::Style {
                color: Some(self.theme.text_muted.to_iced()),
            },
            TextClass::Primary => text::Style {
                color: Some(self.primary()),
            },
            TextClass::Error => text::Style {
                color: Some(self.theme.error.to_iced()),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum TextClass {
    #[default]
    Default,
    Muted,
    Primary,
    Error,
}

// Text input styling
impl text_input::Catalog for WonderlandTheme {
    type Class<'a> = TextInputClass;

    fn default<'a>() -> Self::Class<'a> {
        TextInputClass::Default
    }

    fn style(&self, _class: &Self::Class<'_>, status: text_input::Status) -> text_input::Style {
        let base = text_input::Style {
            background: Background::Color(self.surface()),
            border: Border {
                color: self.theme.border.to_iced(),
                width: 1.0,
                radius: 6.0.into(),
            },
            icon: self.theme.text_muted.to_iced(),
            placeholder: self.theme.text_muted.to_iced(),
            value: self.foreground(),
            selection: self.primary(),
        };

        match status {
            text_input::Status::Active => base,
            text_input::Status::Hovered => text_input::Style {
                border: Border {
                    color: self.theme.border_active.to_iced(),
                    ..base.border
                },
                ..base
            },
            text_input::Status::Focused => text_input::Style {
                border: Border {
                    color: self.primary(),
                    width: 2.0,
                    ..base.border
                },
                ..base
            },
            text_input::Status::Disabled => text_input::Style {
                background: Background::Color(Color {
                    a: 0.5,
                    ..self.surface()
                }),
                ..base
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum TextInputClass {
    #[default]
    Default,
}

// Scrollable styling
impl scrollable::Catalog for WonderlandTheme {
    type Class<'a> = ScrollableClass;

    fn default<'a>() -> Self::Class<'a> {
        ScrollableClass::Default
    }

    fn style(&self, _class: &Self::Class<'_>, status: scrollable::Status) -> scrollable::Style {
        let scrollbar = scrollable::Rail {
            background: Some(Background::Color(Color::TRANSPARENT)),
            border: Border::default(),
            scroller: scrollable::Scroller {
                color: self.theme.border.to_iced(),
                border: Border {
                    radius: 4.0.into(),
                    ..Default::default()
                },
            },
        };

        match status {
            scrollable::Status::Active => scrollable::Style {
                container: container::Style::default(),
                vertical_rail: scrollbar.clone(),
                horizontal_rail: scrollbar,
                gap: None,
            },
            scrollable::Status::Hovered {
                is_horizontal_scrollbar_hovered,
                is_vertical_scrollbar_hovered,
            } => {
                let hovered_rail = scrollable::Rail {
                    scroller: scrollable::Scroller {
                        color: self.theme.text_muted.to_iced(),
                        ..scrollbar.scroller
                    },
                    ..scrollbar.clone()
                };

                scrollable::Style {
                    container: container::Style::default(),
                    vertical_rail: if is_vertical_scrollbar_hovered {
                        hovered_rail.clone()
                    } else {
                        scrollbar.clone()
                    },
                    horizontal_rail: if is_horizontal_scrollbar_hovered {
                        hovered_rail
                    } else {
                        scrollbar
                    },
                    gap: None,
                }
            }
            scrollable::Status::Dragged {
                is_horizontal_scrollbar_dragged,
                is_vertical_scrollbar_dragged,
            } => {
                let dragged_rail = scrollable::Rail {
                    scroller: scrollable::Scroller {
                        color: self.primary(),
                        ..scrollbar.scroller
                    },
                    ..scrollbar.clone()
                };

                scrollable::Style {
                    container: container::Style::default(),
                    vertical_rail: if is_vertical_scrollbar_dragged {
                        dragged_rail.clone()
                    } else {
                        scrollbar.clone()
                    },
                    horizontal_rail: if is_horizontal_scrollbar_dragged {
                        dragged_rail
                    } else {
                        scrollbar
                    },
                    gap: None,
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ScrollableClass {
    #[default]
    Default,
}
