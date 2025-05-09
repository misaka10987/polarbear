use iced::{
    Color, Element,
    widget::{Svg, row, svg, text},
};
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;
use starship_battery::Manager;
use tracing::error;

#[serde_inline_default]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde_inline_default(true)]
    pub enable: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self { enable: true }
    }
}

pub struct Battery {
    pub cfg: Config,
    manager: Option<Manager>,
}

impl Battery {
    pub fn new(cfg: Config) -> Self {
        let manager = Manager::new().inspect_err(|e| error!("{e}")).ok();
        Self { cfg, manager }
    }
    pub fn try_view<T: 'static>(&self) -> anyhow::Result<Element<T>> {
        if !self.cfg.enable {
            return Ok(row![].into());
        }
        let Some(manager) = &self.manager else {
            return Ok(row![].into());
        };
        let batteries = manager.batteries()?;
        let mut rendered = vec![];
        for battery in batteries {
            let battery = battery?;
            rendered.push(render(battery));
        }
        fn render<T: 'static>(battery: starship_battery::Battery) -> Element<'static, T> {
            static CHARGING: &[u8] = include_bytes!("../assets/battery-charging.svg");
            static CONNECTED: &[u8] = include_bytes!("../assets/battery-connected.svg");
            static EMPTY: &[u8] = include_bytes!("../assets/battery-empty.svg");
            static FULL: &[u8] = include_bytes!("../assets/battery-full.svg");
            static LOW: &[u8] = include_bytes!("../assets/battery-low.svg");
            static MEDIUM: &[u8] = include_bytes!("../assets/battery-medium.svg");
            static UNKNOWN: &[u8] = include_bytes!("../assets/battery-unknown.svg");
            let percent = battery.state_of_charge().value;
            let svg = match battery.state() {
                starship_battery::State::Unknown => UNKNOWN,
                starship_battery::State::Charging => CHARGING,
                starship_battery::State::Discharging => {
                    if percent > 0.75 {
                        FULL
                    } else if percent > 0.5 {
                        MEDIUM
                    } else if percent > 0.25 {
                        LOW
                    } else {
                        EMPTY
                    }
                }
                starship_battery::State::Empty => EMPTY,
                starship_battery::State::Full => CONNECTED,
            };
            let icon = Svg::new(svg::Handle::from_memory(svg))
                .style(|_, _| svg::Style {
                    color: Some(Color::WHITE),
                })
                .width(24)
                .height(24);
            let text = text(format!("{}%", (percent * 100.0).round() as i8));
            row![icon, text].into()
        }
        Ok(row(rendered).into())
    }
    pub fn view<T: 'static>(&self) -> Element<T> {
        self.try_view().unwrap_or_else(|e| {
            error!("failed to display battery: {e}");
            row![].into()
        })
    }
}
