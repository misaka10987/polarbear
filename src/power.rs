use anyhow::bail;
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
    pub action: Action,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            action: Action::KDE6,
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
            Message::Logout => self.cfg.action.logout(),
            Message::Hibernate => self.cfg.action.hibernate(),
            Message::Poweroff => self.cfg.action.poweroff(),
            Message::Reboot => self.cfg.action.reboot(),
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
pub enum Action {
    #[serde(rename = "command")]
    Custom {
        logout: String,
        hibernate: String,
        poweroff: String,
        reboot: String,
    },
    KDE6,
}

impl Action {
    pub fn hibernate(&self) -> anyhow::Result<()> {
        bail!("unimplemented")
    }

    pub fn poweroff(&self) -> anyhow::Result<()> {
        bail!("unimplemented")
    }

    pub fn reboot(&self) -> anyhow::Result<()> {
        bail!("unimplemented")
    }

    pub fn logout(&self) -> anyhow::Result<()> {
        let confirm = MessageDialogBuilder::default()
            .set_title("Logout - Polarbear")
            .set_text("Confirm to logout?")
            .confirm()
            .show()?;
        if !confirm {
            return Ok(());
        }
        let cmd = match self {
            Action::Custom { logout, .. } => &logout,
            Action::KDE6 => "qdbus6 org.kde.Shutdown /Shutdown logout",
        };
        run(cmd)?;
        Ok(())
    }
}
