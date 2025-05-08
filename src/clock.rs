use chrono::Local;
use iced::{Element, widget::text};

pub struct Clock {}

impl Clock {
    pub fn new() -> Self {
        Self {}
    }
    pub fn update(&mut self) {}
    pub fn view<'a, T: 'a>(&self) -> Element<'a, T> {
        let time = Local::now();
        let time = time.format("%Y/%m/%d %H:%M:%S");
        text(time.to_string()).into()
    }
}
