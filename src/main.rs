mod clock;

use std::time::Duration;

use clap::Parser;
use clock::Clock;
use iced::{
    Alignment, Color, Element, Length, Task, Theme, time,
    widget::{container, row, text},
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

struct Panel {
    clock: Clock,
}

#[to_layer_message]
#[derive(Debug, Clone)]
enum Message {
    Tick,
}

impl Application for Panel {
    type Message = Message;
    type Flags = ();
    type Theme = Theme;
    type Executor = iced::executor::Default;

    fn new(_flags: ()) -> (Self, Task<Message>) {
        (
            Self {
                clock: Clock::new(),
            },
            Task::none(),
        )
    }

    fn namespace(&self) -> String {
        "Polar Bears' Panel".into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                self.clock.update();
                Task::none()
            }
            _ => unreachable!(),
        }
    }

    fn view(&self) -> Element<Message> {
        row![
            container(text("Polar bears are soluble"))
                .align_x(Alignment::Start)
                .align_y(Alignment::Center)
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(2),
            container(text("Polar bears are soluble"))
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(2),
            container(self.clock.view())
                .align_x(Alignment::End)
                .align_y(Alignment::Center)
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(2),
        ]
        .align_y(Alignment::Center)
        .padding(2)
        .into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        time::every(Duration::from_millis(500)).map(|_| Message::Tick)
    }

    fn style(&self, _: &Self::Theme) -> Appearance {
        Appearance {
            background_color: Color::BLACK,
            text_color: Color::WHITE,
        }
    }
}
