use crate::clock::Clock;
use iced::{
    Alignment, Element, Length,
    widget::{container, row, text},
};

pub struct Panel {
    clock: Clock,
}

#[derive(Clone, Debug)]
pub enum Message {
    Tick,
}

impl Panel {
    pub fn new(clock: Clock) -> Self {
        Self { clock }
    }

    pub fn update(&mut self, _: Message) {}

    pub fn view(&self) -> Element<Message> {
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
}
