mod clock;

use std::{
    fs,
    path::{Path, PathBuf},
    time::Duration,
};

use clap::Parser;
use clock::Clock;
use dirs::config_dir;
use iced::{
    Alignment, Color, Element, Font, Length, Pixels, Task, Theme, time,
    widget::{container, row, text},
};
use iced_layershell::{
    Appearance, Application,
    reexport::Anchor,
    settings::{LayerShellSettings, Settings},
    to_layer_message,
};
use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

#[derive(Parser)]
#[command(version)]
struct PolarBear {
    #[arg(short, long)]
    pub config: Option<PathBuf>,
}

impl PolarBear {
    pub fn try_load_config(&self) -> Config {
        fn load_config(path: impl AsRef<Path>) -> anyhow::Result<Config> {
            let file = fs::read_to_string(path)?;
            let val = toml::from_str(&file)?;
            Ok(val)
        }
        let mut attempts: Vec<PathBuf> = vec![];
        if let Some(path) = &self.config {
            attempts.push(path.clone());
        }
        if let Some(dir) = config_dir() {
            attempts.push(dir.join("polarbear").join("config.toml"));
            attempts.push(dir.join("polarbear.toml"));
        }
        for attempt in attempts {
            let res = load_config(&attempt);
            match res {
                Ok(cfg) => {
                    debug!("loaded config at {attempt:?} : {cfg:?}");
                    return cfg;
                }
                Err(e) => debug!("load config at {attempt:?} failed: {e}"),
            }
        }
        warn!("all attempts to load config failed, using default");
        Default::default()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Config {
    pub clock: clock::Config,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            clock: Default::default(),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let args = PolarBear::parse();
    tracing_subscriber::fmt().init();
    let cfg = args.try_load_config();
    start(cfg)?;
    Ok(())
}

fn start(cfg: Config) -> anyhow::Result<()> {
    let layershell = LayerShellSettings {
        size: Some((0, 32)),
        exclusive_zone: 32,
        anchor: Anchor::Top | Anchor::Left | Anchor::Right,
        ..Default::default()
    };
    let settings = Settings {
        flags: cfg,
        layer_settings: layershell,
        antialiasing: true,
        id: None,
        fonts: Vec::new(),
        default_font: Font::default(),
        default_text_size: Pixels(16.0),
        virtual_keyboard_support: None,
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
    type Flags = Config;
    type Theme = Theme;
    type Executor = iced::executor::Default;

    fn new(flags: Self::Flags) -> (Self, Task<Message>) {
        (
            Self {
                clock: Clock::new(flags.clock.clone()),
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
