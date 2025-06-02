use std::fmt::Display;

use iced::{Element, Length, widget::Column};
use iced_aw::{TabBar, TabLabel, iced_fonts};

fn main() -> iced::Result {
    iced::application("Gruber", State::update, State::view)
        .font(iced_fonts::REQUIRED_FONT_BYTES)
        .resizable(false)
        .window_size((400.0, 400.0))
        .run()
}

/// State transitions
#[derive(Debug, Clone)]
enum Message {
    TabSelected(Tab),
}

/// Global app state
#[derive(Debug, Default)]
struct State {
    active_tab: Tab,
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::TabSelected(index) => self.active_tab = index,
        }
    }

    fn view(&self) -> Element<Message> {
        Column::new()
            .push(
                // Build the tab bar
                Tab::iter()
                    .fold(TabBar::new(Message::TabSelected), |tab_bar, tab| {
                        tab_bar.push(tab, TabLabel::Text(tab.to_string()))
                    })
                    .set_active_tab(&self.active_tab)
                    // Fill the entire screen evenly
                    .tab_width(Length::FillPortion(Tab::iter().count() as u16))
                    .padding(5.0)
                    .text_size(32.0),
            )
            .into()
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
enum Tab {
    #[default]
    Weather,
    Transit,
}

impl Tab {
    fn iter() -> impl Iterator<Item = Self> {
        [Self::Weather, Self::Transit].into_iter()
    }
}

impl Display for Tab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tab::Weather => write!(f, "Weather"),
            Tab::Transit => write!(f, "Transit"),
        }
    }
}
