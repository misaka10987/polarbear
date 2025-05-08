use clap::Parser;
use iced::{
    Alignment, Color, Element, Length, Task, Theme,
    widget::{container, text},
};
use iced_layershell::{
    Appearance, Application,
    reexport::Anchor,
    settings::{LayerShellSettings, Settings},
    to_layer_message,
};

#[derive(Parser)]
#[command(version)]
struct Args {}

fn main() -> anyhow::Result<()> {
    Args::parse();
    tracing_subscriber::fmt().init();
    let layershell = LayerShellSettings {
        size: Some((0, 32)),
        exclusive_zone: 32,
        anchor: Anchor::Top | Anchor::Left | Anchor::Right,
        ..Default::default()
    };
    let settings = Settings {
        layer_settings: layershell,
        antialiasing: true,
        ..Default::default()
    };
    Panel::run(settings)?;
    Ok(())
}

struct Panel {}

#[to_layer_message]
#[derive(Debug, Clone)]
enum Message {}

impl Application for Panel {
    type Message = Message;
    type Flags = ();
    type Theme = Theme;
    type Executor = iced::executor::Default;

    fn new(_flags: ()) -> (Self, Task<Message>) {
        (Self {}, Task::none())
    }

    fn namespace(&self) -> String {
        "Polar Bears' Panel".into()
    }

    fn update(&mut self, _: Message) -> Task<Message> {
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        container(text("Polar bears are soluble").size(16))
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(2)
            .into()
    }

    fn style(&self, _: &Self::Theme) -> Appearance {
        Appearance {
            background_color: Color::BLACK,
            text_color: Color::WHITE,
        }
    }
}
