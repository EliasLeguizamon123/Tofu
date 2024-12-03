use iced::{
    advanced::widget::text, alignment::{Horizontal, Vertical}, executor, widget::Text, Application, Command, Element, Length, Settings, Subscription, Theme
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

struct CustomTheme(Theme);

#[derive(Debug, Clone)]
enum Message {
    PriceUpdated(String),
    Tick,
}

impl text::StyleSheet for CustomTheme {
    type Style = ();

    fn appearance(&self, _style: Self::Style) -> text::Appearance {
        text::Appearance {
            color: Some(self.0.palette().text),
        }
    }
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
        time::every(std::time::Duration::from_secs(1)).map(|_| Message::Tick)
    }

    fn view(&self) -> Element<Self::Message> {
        let price = self
            .current_price
            .as_deref()
            .and_then(|price| price.parse::<f64>().ok()) 
            .map(|price| format!("{:.3}", price))
            .unwrap_or_else(|| "0".to_string());


        Text::new(price + "$")
            .size(60)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(self.theme().palette().text)
            .into()
    }
    
    fn theme(&self) -> Theme {
        Theme::custom(
            String::from("Tofu Dark"),
            iced::theme::Palette {
                background: iced::Color::from_rgb8(0x28, 0x24, 0x22), // #282422 - Marrón oscuro (base cálida)
                primary: iced::Color::from_rgb8(0xEB, 0x9B, 0x34),    // #EB9B34 - Naranja cálido (tofu marinado)
                text: iced::Color::from_rgb8(0xE8, 0xD4, 0xA0),       // #E8D4A0 - Beige cálido (tofu suave)
                success: iced::Color::from_rgb8(0xA8, 0xC2, 0x34),    // #A8C234 - Verde cálido (hierbas frescas)
                danger: iced::Color::from_rgb8(0xD7, 0x4E, 0x4E),     // #D74E4E - Rojo cálido (pimienta cayena)
            },
        )
    }
    
    

}

async fn fetch_price() -> String {
    match webhook::connect_to_ticker("xrpusdt").await {
        Some(ticker) => ticker.last_price,
        None => "0$".to_string(),
    }
}
