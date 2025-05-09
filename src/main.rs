mod clock;
mod cmd;
mod panel;
mod power;

use std::{
    fs, io,
    ops::Deref,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::Arc,
    time::Duration,
};

use clap::Parser;
use clock::Clock;
use cmd::SubCommand;
use dirs::config_dir;
use iced::{Color, Element, Font, Pixels, Task, Theme, time};
use iced_layershell::{
    Appearance, Application,
    reexport::Anchor,
    settings::{LayerShellSettings, Settings},
    to_layer_message,
};
use panel::Panel;
use power::Power;
use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

fn run(shell_cmd: &str) -> io::Result<Output> {
    Command::new("sh").arg("-c").arg(shell_cmd).output()
}

/// Polar bears' panel for wayland.
#[derive(Parser)]
#[command(version)]
struct PolarBear {
    /// Path to the configuration file.
    ///
    /// If not speficied, would look up sequentially the following paths:
    ///
    /// - `$XDG_CONFIG_HOME/polarbear/config.toml`
    ///
    /// - `$XDG_CONFIG_HOME/polarbear.toml`
    ///
    /// If environment `$XDG_CONFIG_HOME` is not specified,
    /// would use `$HOME/.config` instead.
    #[arg(short, long)]
    pub config: Option<PathBuf>,
    /// CLI command to run with.
    ///
    /// If not specified, launch the panel.
    #[command(subcommand)]
    pub cmd: Option<SubCommand>,
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
    pub tick_period: u64,
    pub clock: clock::Config,
    pub power: power::Config,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            tick_period: 500,
            clock: Default::default(),
            power: Default::default(),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let args = PolarBear::parse();
    if let Some(cmd) = &args.cmd {
        return cmd.run();
    }
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
    App::run(settings)?;
    Ok(())
}

struct AppInner {
    pub cfg: Config,
    panel: Panel,
}

#[derive(Clone)]
struct App(Arc<AppInner>);

impl Deref for App {
    type Target = AppInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[to_layer_message]
#[derive(Clone, Debug)]
enum AppMessage {
    Only(panel::Message),
}

impl Application for App {
    type Message = AppMessage;
    type Flags = Config;
    type Theme = Theme;
    type Executor = iced::executor::Default;

    fn new(cfg: Self::Flags) -> (Self, Task<AppMessage>) {
        let clock = Clock::new(cfg.clock.clone());
        let power = Power::new(cfg.power.clone());
        let panel = Panel::new(clock, power);
        (Self(Arc::new(AppInner { cfg, panel })), Task::none())
    }

    fn namespace(&self) -> String {
        "Polar Bears' Panel".into()
    }

    fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        let AppMessage::Only(msg) = message else {
            unreachable!()
        };
        let app = self.clone();
        let fut = async move {
            app.panel.update(msg).await;
        };
        Task::future(fut).discard()
    }

    fn view(&self) -> Element<AppMessage> {
        self.panel.view().map(AppMessage::Only)
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        time::every(Duration::from_millis(self.cfg.tick_period))
            .map(|_| AppMessage::Only(panel::Message::Tick))
    }

    fn style(&self, _: &Self::Theme) -> Appearance {
        Appearance {
            background_color: Color::BLACK,
            text_color: Color::WHITE,
        }
    }
}
