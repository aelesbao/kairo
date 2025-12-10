use iced::{
    Alignment,
    Background,
    Element,
    Length,
    Task,
    Theme,
    advanced::graphics::text::cosmic_text::skrifa::raw::collections::int_set::Domain,
    border,
    widget::{button, center, column, container, image, row, scrollable, svg, text, tooltip},
    window,
};
use kairo_core::{Url, UrlHandlerApp};
use unicode_segmentation::UnicodeSegmentation;

// TODO: fetch from cargo metadata
const APP_ID: &str = "io.github.aelesbao.Kairo";

#[cfg(target_os = "macos")]
const WIN_SIZE: [f32; 2] = [640.0, 210.0];
#[cfg(not(target_os = "macos"))]
const WIN_SIZE: [f32; 2] = [640.0, 190.0];
#[cfg(target_os = "macos")]
const WIN_MIN_SIZE: [f32; 2] = [480.0, 210.0];
#[cfg(not(target_os = "macos"))]
const WIN_MIN_SIZE: [f32; 2] = [480.0, 190.0];
const WIN_MAX_SIZE: [f32; 2] = [1280.0, 480.0];

const APP_FONT_SIZE: u32 = 12;
const URL_FONT_SIZE: u32 = 14;
const TOOLTIP_FONT_SIZE: u32 = 10;

const OUTER_SPACING: f32 = 20.0;
const INNER_SPACING: f32 = 10.0;
const BORDER_RADIUS: f32 = 10.0;

const ICON_SIZE: u16 = 64;

const UNKOWN_APP_ICON_BYTES: &[u8] = include_bytes!("../assets/unknown.svg");

pub fn run(url: Url, apps: Vec<UrlHandlerApp>, explain: bool) -> iced::Result {
    log::info!("Launching UI for URL handler selection");
    application(url, apps, explain).run()
}

fn application(
    url: Url,
    apps: Vec<UrlHandlerApp>,
    explain: bool,
) -> iced::Application<impl iced::Program<Message = Message>> {
    let settings = iced::Settings {
        id: Some(APP_ID.to_string()),
        default_text_size: APP_FONT_SIZE.into(),
        ..Default::default()
    };

    let window = window::Settings {
        size: WIN_SIZE.into(),
        max_size: Some(WIN_MAX_SIZE.into()),
        min_size: Some(WIN_MIN_SIZE.into()),
        resizable: true,
        position: window::Position::Centered,
        platform_specific: platform_settings(APP_ID.to_string()),
        ..Default::default()
    };

    iced::application(
        move || App::new(url.clone(), apps.clone(), explain),
        App::update,
        App::view,
    )
    .title(App::title)
    .theme(App::theme)
    .settings(settings)
    .window(window)
}

#[cfg(target_os = "linux")]
fn platform_settings(application_id: String) -> window::settings::PlatformSpecific {
    window::settings::PlatformSpecific {
        application_id,
        ..Default::default()
    }
}

#[cfg(target_os = "macos")]
fn platform_settings(_application_id: String) -> window::settings::PlatformSpecific {
    window::settings::PlatformSpecific {
        title_hidden: false,
        titlebar_transparent: true,
        fullsize_content_view: true,
    }
}

#[cfg(not(any(target_os = "macos", target_os = "linux",)))]
fn platform_settings(_application_id: String) -> window::settings::PlatformSpecific {
    Default::default()
}

#[derive(Debug, Clone)]
enum Message {
    OpenWithApp(UrlHandlerApp),
}

struct App {
    url: Url,
    apps: Vec<UrlHandlerApp>,
    explain: bool,
}

impl App {
    fn new(url: Url, apps: Vec<UrlHandlerApp>, explain: bool) -> (Self, Task<Message>) {
        let app = Self { url, apps, explain };
        (app, Task::none())
    }

    fn title(&self) -> String {
        "Select Application to Open URL - Kairo".to_string()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OpenWithApp(app) => match app.open_url(self.url.clone()) {
                Ok(_) => window::latest().and_then(window::close),
                Err(e) => {
                    log::error!("Failed to open URL with '{}': {}", app.name, e);
                    Task::none()
                }
            },
        }
    }

    fn view(&self) -> Element<'_, Message> {
        log::info!("Rendering URL handler selection UI");

        let apps_buttons = self.apps.iter().map(|app| {
            let app_icon = app_icon(app, ICON_SIZE);
            let app_name = text(truncate_with_ellipsis(&app.name, 12)).center();

            let label = column![app_icon, app_name]
                .spacing(INNER_SPACING)
                .width(100)
                .align_x(Alignment::Center);

            let app_button = button(label)
                .padding(INNER_SPACING)
                .style(app_button_style)
                .on_press(Message::OpenWithApp(app.clone()));

            tooltip(
                app_button,
                text(&app.name).size(TOOLTIP_FONT_SIZE),
                tooltip::Position::FollowCursor,
            )
            .gap(INNER_SPACING)
            .style(app_tooltip_style)
            .into()
        });

        let scrollbar = scrollable::Scrollbar::new().width(2).scroller_width(5);
        let apps_container = scrollable::Scrollable::with_direction(
            row(apps_buttons).spacing(OUTER_SPACING),
            scrollable::Direction::Horizontal(scrollbar),
        );

        let url_text = text(self.url.as_str())
            .size(URL_FONT_SIZE)
            .style(text::primary)
            .align_x(Alignment::Center)
            .width(Length::Fill)
            .wrapping(text::Wrapping::Glyph);

        let content: Element<_> = column![apps_container, url_text]
            .spacing(OUTER_SPACING)
            .padding(OUTER_SPACING)
            .align_x(Alignment::Center)
            .into();

        let content = if self.explain {
            content.explain(iced::Color::WHITE)
        } else {
            content
        };

        center(content).into()
    }

    fn theme(&self) -> Theme {
        Theme::TokyoNight
    }
}

fn app_icon<T>(app: &UrlHandlerApp, icon_size: u16) -> Element<'_, T> {
    let length = iced::Length::from(icon_size.to_u32());
    match app.icon_path(icon_size) {
        Some(path) if path.extension().is_some_and(|ext| ext.eq("svg")) => {
            log::trace!("Found svg icon for {}: {:?}", app.appid, path);
            svg(path).height(length).width(length).into()
        }
        Some(path) => {
            log::trace!("Found standard icon for {}: {:?}", app.appid, path);
            image(path).height(length).width(length).into()
        }
        None => {
            log::trace!("No icon found for {}, using placeholder", app.appid);
            let handle = svg::Handle::from_memory(UNKOWN_APP_ICON_BYTES);
            svg(handle)
                .height(length)
                .width(length)
                .style(unknown_app_icon_style)
                .into()
        }
    }
}

fn app_button_style(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();
    let style = button::Style {
        background: None,
        text_color: palette.secondary.base.text,
        border: border::rounded(BORDER_RADIUS),
        ..button::Style::default()
    };

    match status {
        button::Status::Active | button::Status::Pressed => style,
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(palette.secondary.weak.color).scale_alpha(0.2)),
            ..style
        },
        button::Status::Disabled => button::Style {
            background: style
                .background
                .map(|background| background.scale_alpha(0.5)),
            text_color: style.text_color.scale_alpha(0.5),
            ..style
        },
    }
}

fn app_tooltip_style(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();
    container::Style {
        background: Some(palette.background.base.color.into()),
        border: border::rounded(BORDER_RADIUS),
        ..container::Style::default()
    }
}

fn unknown_app_icon_style(theme: &Theme, _status: svg::Status) -> svg::Style {
    let palette = theme.extended_palette();
    svg::Style {
        color: Some(palette.secondary.weak.text),
    }
}

pub fn truncate_with_ellipsis(s: &str, max: usize) -> String {
    let g = s.graphemes(true).collect::<Vec<_>>();
    if g.len() <= max {
        return s.to_owned();
    }

    let mut out = g.into_iter().take(max).collect::<String>();
    out.push_str("...");
    out
}
