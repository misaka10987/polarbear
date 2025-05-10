use anyhow::bail;
use iced::{
    Border, Color, Element, Shadow,
    widget::{Svg, button, row, svg},
};
use native_dialog::MessageDialogBuilder;
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;
use tracing::error;

use crate::run;

#[serde_inline_default]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde_inline_default(true)]
    pub enable: bool,
    #[serde(default)]
    pub action: Action,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            enable: true,
            action: Default::default(),
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

    pub async fn update(&self, message: Message) {
        let res = match &message {
            Message::Logout => self.cfg.action.logout().await,
            Message::Hibernate => self.cfg.action.hibernate().await,
            Message::Poweroff => self.cfg.action.poweroff().await,
            Message::Reboot => self.cfg.action.reboot().await,
        };
        if let Err(e) = res {
            error!("failed to handle {message:?}: {e}");
        }
    }

    pub fn view(&self) -> Element<Message> {
        static HIBERNATE: &[u8] = include_bytes!("../assets/hibernate.svg");
        static LOGOUT: &[u8] = include_bytes!("../assets/logout.svg");
        static POWEROFF: &[u8] = include_bytes!("../assets/poweroff.svg");
        static REBOOT: &[u8] = include_bytes!("../assets/reboot.svg");

        if !self.cfg.enable {
            return row![].into();
        }

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

async fn confirm(title: &str, text: &str) -> anyhow::Result<bool> {
    let confirm = MessageDialogBuilder::default()
        .set_title(title)
        .set_text(text)
        .confirm()
        .spawn()
        .await?;
    Ok(confirm)
}

impl Action {
    pub async fn hibernate(&self) -> anyhow::Result<()> {
        bail!("unimplemented")
    }

    pub async fn poweroff(&self) -> anyhow::Result<()> {
        if !confirm("Poweroff - Polarbear", "Confirm poweroff?").await? {
            return Ok(());
        }
        let cmd = match self {
            Action::Custom { logout, .. } => &logout,
            Action::KDE6 => "qdbus6 org.kde.Shutdown /Shutdown logoutAndShutdown",
        };
        run(cmd)?;
        Ok(())
    }

    pub async fn reboot(&self) -> anyhow::Result<()> {
        if !confirm("Reboot - Polarbear", "Confirm reboot?").await? {
            return Ok(());
        }
        let cmd = match self {
            Action::Custom { logout, .. } => &logout,
            Action::KDE6 => "qdbus6 org.kde.Shutdown /Shutdown logoutAndReboot",
        };
        run(cmd)?;
        Ok(())
    }

    pub async fn logout(&self) -> anyhow::Result<()> {
        if !confirm("Logout - Polarbear", "Confirm logout?").await? {
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

impl Default for Action {
    fn default() -> Self {
        Self::KDE6
    }
}
