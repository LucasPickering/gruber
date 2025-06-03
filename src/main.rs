mod config;
mod services;

use crate::{
    config::Config,
    services::weather::{self, Forecast},
};
use iced::{
    Element, Length, Task,
    widget::{Column, text},
};
use iced_aw::{TabBar, TabLabel, iced_fonts};
use std::fmt::Display;

fn main() -> anyhow::Result<()> {
    let config = Config::load()?;

    iced::application("Gruber", State::update, State::view)
        // .subscription(State::subscription)
        .font(iced_fonts::REQUIRED_FONT_BYTES)
        .resizable(false)
        .window_size(config.window_size)
        .run_with(|| {
            (State::new(config), Task::done(Message::WeatherFetchStart))
        })?;

    Ok(())
}

/// State transitions
#[derive(Debug, Clone)]
enum Message {
    TabSelected(Tab),
    WeatherFetchStart,
    WeatherFetchEnd(Forecast),
}

/// Global app state
#[derive(Debug)]
struct State {
    config: Config,
    active_tab: Tab,
    forecast: Option<Forecast>,
}

impl State {
    fn new(config: Config) -> Self {
        Self {
            config,
            active_tab: Tab::Weather,
            forecast: None,
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::TabSelected(index) => self.active_tab = index,
            Message::WeatherFetchStart => {
                // Spawn a task to fetch weather
                return services::fallible_task(
                    weather::fetch_weather(&self.config),
                    Message::WeatherFetchEnd,
                );
            }
            Message::WeatherFetchEnd(forecast) => {
                self.forecast = Some(forecast)
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        // Build the tab bar
        let tabs = Tab::iter()
            .fold(TabBar::new(Message::TabSelected), |tab_bar, tab| {
                tab_bar.push(tab, TabLabel::Text(tab.to_string()))
            })
            .set_active_tab(&self.active_tab)
            // Fill the entire screen evenly
            .tab_width(Length::FillPortion(Tab::iter().count() as u16))
            .padding(5.0)
            .text_size(32.0);
        let content = match self.active_tab {
            Tab::Weather => {
                if let Some(forecast) = &self.forecast {
                    view_weather(forecast)
                } else {
                    text("Loading...").into()
                }
            }
            Tab::Transit => text("TODO").into(),
        };
        Column::new().push(tabs).push(content).into()
    }
}

fn view_weather(forecast: &weather::Forecast) -> Element<'_, Message> {
    let now = forecast.now();
    text(format!("{} / {}", now.temperature(), now.prob_of_precip())).into()
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Tab {
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
