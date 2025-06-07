mod config;
mod services;
mod view;

use crate::{
    config::Config,
    services::{ExternalData, FetchedData, transit::Transit, weather::Weather},
};
use iced::{Subscription, Task, Theme, window};
use iced_aw::iced_fonts;
use std::{fmt::Display, time::Duration};

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
        .subscription(State::check_data_subscription)
        .settings(iced::Settings {
            default_text_size: 24.0.into(),
            ..iced::Settings::default()
        })
        .font(iced_fonts::REQUIRED_FONT_BYTES)
        .resizable(false)
        .window(window_settings)
        .theme(|_| Theme::TokyoNightStorm)
        .run_with(|| (State::new(config), Task::done(Message::CheckData)))?;

    Ok(())
}

/// State transitions
#[derive(Debug)]
enum Message {
    /// Periodically check all data to see if it's stale. Anything that is will
    /// be refetched
    CheckData,
    TabSelected(Tab),
    WeatherFetched(FetchedData<<Weather as ExternalData>::Data>),
    TransitFetched(FetchedData<<Transit as ExternalData>::Data>),
}

/// Global app state
#[derive(Debug)]
struct State {
    active_tab: Tab,
    weather: Weather,
    transit: Transit,
}

impl State {
    fn new(config: Config) -> Self {
        let weather = Weather::new(&config);
        let transit = Transit::new(&config);
        Self {
            active_tab: Tab::Weather,
            weather,
            transit,
        }
    }

    /// Update state according to an incoming message
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::CheckData => {
                // Check all data sources in parallel
                return Task::batch([
                    self.weather.fetch_if_needed(),
                    self.transit.fetch_if_needed(),
                ]);
            }
            Message::TabSelected(index) => self.active_tab = index,
            Message::WeatherFetched(data) => self.weather.set_data(data),
            Message::TransitFetched(data) => self.transit.set_data(data),
        }
        Task::none()
    }

    /// Create a subscription that will periodically send a `CheckData` message
    fn check_data_subscription(&self) -> Subscription<Message> {
        iced::time::every(Duration::from_secs(1)).map(|_| Message::CheckData)
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
