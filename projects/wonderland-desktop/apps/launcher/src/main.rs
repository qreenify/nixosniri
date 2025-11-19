//! Wonderland Launcher
//!
//! Themeable application launcher for Wayland.

use iced::widget::{column, container, row, scrollable, text, text_input, Column};
use iced::{Element, Length, Task};
use wonderland_theme::{ContainerClass, ThemeLoader, WonderlandTheme};

fn main() -> iced::Result {
    tracing_subscriber::fmt::init();

    iced::application("Wonderland Launcher", App::update, App::view)
        .run_with(App::new)
}

struct App {
    _theme: WonderlandTheme,
    search_query: String,
    apps: Vec<AppEntry>,
    filtered_apps: Vec<usize>,
    selected: usize,
}

#[derive(Debug, Clone)]
struct AppEntry {
    name: String,
    exec: String,
    icon: Option<String>,
}

#[derive(Debug, Clone)]
enum Message {
    SearchChanged(String),
    Launch(usize),
    SelectNext,
    SelectPrev,
    Exit,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        // Load theme
        let loader = ThemeLoader::default();
        let theme_data = loader.load_current().unwrap_or_else(|e| {
            tracing::warn!("Failed to load theme: {}, using fallback", e);
            // Fallback theme
            wonderland_theme::Theme {
                name: "fallback".to_string(),
                path: std::path::PathBuf::new(),
                foreground: wonderland_theme::Color::from_hex("#cdd6f4").unwrap(),
                background: wonderland_theme::Color::from_hex("#1e1e2e").unwrap(),
                primary: wonderland_theme::Color::from_hex("#89b4fa").unwrap(),
                secondary: wonderland_theme::Color::from_hex("#74c7ec").unwrap(),
                surface: wonderland_theme::Color::from_hex("#313244").unwrap(),
                error: wonderland_theme::Color::from_hex("#f38ba8").unwrap(),
                warning: wonderland_theme::Color::from_hex("#f9e2af").unwrap(),
                success: wonderland_theme::Color::from_hex("#a6e3a1").unwrap(),
                border: wonderland_theme::Color::from_hex("#45475a").unwrap(),
                border_active: wonderland_theme::Color::from_hex("#89b4fa").unwrap(),
                text_muted: wonderland_theme::Color::from_hex("#6c7086").unwrap(),
            }
        });

        let theme = WonderlandTheme::new(theme_data);

        // TODO: Load actual desktop entries
        let apps = vec![
            AppEntry {
                name: "Ghostty".to_string(),
                exec: "ghostty".to_string(),
                icon: None,
            },
            AppEntry {
                name: "Firefox".to_string(),
                exec: "firefox".to_string(),
                icon: None,
            },
            AppEntry {
                name: "Brave".to_string(),
                exec: "brave".to_string(),
                icon: None,
            },
            AppEntry {
                name: "Nautilus".to_string(),
                exec: "nautilus".to_string(),
                icon: None,
            },
            AppEntry {
                name: "VSCodium".to_string(),
                exec: "codium".to_string(),
                icon: None,
            },
        ];

        let filtered_apps: Vec<usize> = (0..apps.len()).collect();

        (
            Self {
                _theme: theme,
                search_query: String::new(),
                apps,
                filtered_apps,
                selected: 0,
            },
            text_input::focus(text_input::Id::new("search")),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SearchChanged(query) => {
                self.search_query = query.clone();
                self.filter_apps(&query);
                self.selected = 0;
            }
            Message::Launch(index) => {
                if let Some(&app_index) = self.filtered_apps.get(index) {
                    let app = &self.apps[app_index];
                    tracing::info!("Launching: {}", app.exec);
                    // TODO: Actually launch the app
                    let _ = std::process::Command::new("sh")
                        .arg("-c")
                        .arg(&app.exec)
                        .spawn();
                    return iced::exit();
                }
            }
            Message::SelectNext => {
                if self.selected < self.filtered_apps.len().saturating_sub(1) {
                    self.selected += 1;
                }
            }
            Message::SelectPrev => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
            }
            Message::Exit => {
                return iced::exit();
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let search = text_input("Search...", &self.search_query)
            .id(text_input::Id::new("search"))
            .on_input(Message::SearchChanged)
            .padding(12)
            .size(18);

        let results: Column<Message> = self
            .filtered_apps
            .iter()
            .enumerate()
            .fold(Column::new().spacing(4), |col, (i, &app_index)| {
                let app = &self.apps[app_index];
                let is_selected = i == self.selected;

                let item = container(
                    row![text(&app.name).size(16)]
                        .padding(12)
                        .width(Length::Fill),
                );

                col.push(item)
            });

        let content = column![search, scrollable(results).height(Length::Fill)]
            .spacing(8)
            .padding(16);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn filter_apps(&mut self, query: &str) {
        if query.is_empty() {
            self.filtered_apps = (0..self.apps.len()).collect();
        } else {
            let query_lower = query.to_lowercase();
            self.filtered_apps = self
                .apps
                .iter()
                .enumerate()
                .filter(|(_, app)| app.name.to_lowercase().contains(&query_lower))
                .map(|(i, _)| i)
                .collect();
        }
    }
}
