mod config;
mod services;
mod view;

use crate::{
    config::Config,
    services::weather::{self, Forecast},
};
use iced::{Task, window};
use iced_aw::iced_fonts;
use std::fmt::Display;

fn main() -> anyhow::Result<()> {
    let config = Config::load()?;

    let window_settings = window::Settings {
        size: config.window_size.into(),
        position: config
            .window_position
            .map(|position| window::Position::Specific(position.into()))
            .unwrap_or_default(),
        ..window::Settings::default()
    };
    iced::application("Gruber", State::update, view::view)
        // .subscription(State::subscription)
        .font(iced_fonts::REQUIRED_FONT_BYTES)
        .resizable(false)
        .window(window_settings)
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
