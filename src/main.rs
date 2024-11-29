use iced::{
    alignment::{Horizontal, Vertical}, executor, widget::Text, Application, Color, Command, Element, Length, Settings, Subscription, Theme
};
use iced::time;

mod structs;
mod webhook;

pub fn main() -> iced::Result {
    TofuApp::run(Settings {
        window: iced::window::Settings {
            size: iced::Size {
                width: 800.0,
                height: 600.0,
            },
            ..Default::default()
        },
        ..Default::default()
    })
}

struct TofuApp {
    current_price: Option<String>,
}

#[derive(Debug, Clone)]
enum Message {
    PriceUpdated(String),
    Tick,
}

impl Application for TofuApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            TofuApp {
                current_price: None,
            },
            Command::perform(fetch_price(), Message::PriceUpdated),
        )
    }

    fn title(&self) -> String {
        String::from("Tofu")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::PriceUpdated(price) => {
                self.current_price = Some(price);
                Command::none()
            }
            Message::Tick => {
                Command::perform(fetch_price(), Message::PriceUpdated)
            }
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        time::every(std::time::Duration::from_secs(5)).map(|_| Message::Tick)
    }

    fn view(&self) -> Element<Self::Message> {
        let price = self
            .current_price
            .as_deref()
            .unwrap_or("Cargando precio...");

        Text::new(price)
            .size(60)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            // white text
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::default()
    }
}

async fn fetch_price() -> String {
    match webhook::connect_to_ticker("xrpusdt").await {
        Some(ticker) => ticker.last_price,
        None => "0$".to_string(),
    }
}
