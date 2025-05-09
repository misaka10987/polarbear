use crate::{
    battery::Battery,
    clock::Clock,
    power::{self, Power},
};
use iced::{
    Alignment, Element, Length,
    widget::{container, row, text},
};

pub struct Panel {
    battery: Battery,
    clock: Clock,
    power: Power,
}

#[derive(Clone, Debug)]
pub enum Message {
    Tick,
    Power(power::Message),
}

impl Panel {
    pub fn new(clock: Clock, power: Power, battery: Battery) -> Self {
        Self {
            clock,
            power,
            battery,
        }
    }

    pub async fn update(&self, message: Message) {
        match message {
            Message::Tick => {}
            Message::Power(msg) => self.power.update(msg).await,
        }
    }

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
            container(
                row![
                    self.battery.view(),
                    self.clock.view(),
                    self.power.view().map(Message::Power)
                ]
                .spacing(8)
            )
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
