use chrono::Local;
use iced::{Element, widget::text};
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;

#[serde_inline_default]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde_inline_default("%Y/%m/%d %H:%M:%S".into())]
    pub format: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            format: "%Y/%m/%d %H:%M:%S".into(),
        }
    }
}

pub struct Clock {
    pub cfg: Config,
}

impl Clock {
    pub fn new(cfg: Config) -> Self {
        Self { cfg }
    }
    pub fn update(&mut self) {}
    pub fn view<'a, T: 'a>(&self) -> Element<'a, T> {
        let time = Local::now();
        let time = time.format(&self.cfg.format);
        text(time.to_string()).into()
    }
}
