use anyhow::anyhow;
use iced::{
    Border, Color, Element, Shadow,
    widget::{Svg, button, row, svg},
};
use native_dialog::MessageDialogBuilder;
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::run;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub logout: LogoutCommand,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            logout: LogoutCommand::KDE6,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    Logout,
    Hibernate,
    Reboot,
    Poweroff,
}

pub struct Power {
    pub cfg: Config,
}

impl Power {
    pub fn new(cfg: Config) -> Self {
        Self { cfg }
    }

    pub fn update(&mut self, message: Message) {
        dbg!(&message);
        let res = match message {
            Message::Logout => self.cfg.logout.run(),
            _ => Err(anyhow!("unimplemented")),
        };
        if let Err(e) = res {
            error!("{e}");
        }
    }

    pub fn view(&self) -> Element<Message> {
        static HIBERNATE: &'static [u8] = include_bytes!("../assets/hibernate.svg");
        static LOGOUT: &'static [u8] = include_bytes!("../assets/logout.svg");
        static POWEROFF: &'static [u8] = include_bytes!("../assets/poweroff.svg");
        static REBOOT: &'static [u8] = include_bytes!("../assets/reboot.svg");

        fn item(icon: &'static [u8], action: Message) -> Element<'static, Message> {
            let logout = Svg::new(svg::Handle::from_memory(icon))
                .style(|_, _| svg::Style {
                    color: Some(Color::WHITE),
                })
                .width(20)
                .height(20);
            let logout = button(logout)
                .on_press(action)
                .padding(0)
                .style(|_, _| button::Style {
                    background: None,
                    text_color: Color::WHITE,
                    border: Border::default().rounded(4),
                    shadow: Shadow::default(),
                });
            logout.into()
        }

        row![
            item(HIBERNATE, Message::Hibernate),
            item(REBOOT, Message::Reboot),
            item(POWEROFF, Message::Poweroff),
            item(LOGOUT, Message::Logout)
        ]
        .spacing(4)
        .into()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LogoutCommand {
    #[serde(rename = "command")]
    Custom(String),
    KDE6,
}

impl LogoutCommand {
    pub fn run(&self) -> anyhow::Result<()> {
        let confirm = MessageDialogBuilder::default()
            .set_title("Logout - Polarbear")
            .set_text("Confirm to logout?")
            .confirm()
            .show()?;
        if !confirm {
            return Ok(());
        }
        let cmd = match self {
            LogoutCommand::Custom(x) => x,
            LogoutCommand::KDE6 => "qdbus6 org.kde.Shutdown /Shutdown logout",
        };
        run(cmd)?;
        Ok(())
    }
}
