use crate::{
    Message, State, Tab,
    services::weather::{Forecast, ForecastPeriod},
};
use iced::{
    Element, Length,
    widget::{Column, Text, text},
};
use iced_aw::{TabBar, TabLabel};

pub fn view(state: &State) -> Element<Message> {
    // Build the tab bar
    let tabs = Tab::iter()
        .fold(TabBar::new(Message::TabSelected), |tab_bar, tab| {
            tab_bar.push(tab, TabLabel::Text(tab.to_string()))
        })
        .set_active_tab(&state.active_tab)
        // Fill the entire screen evenly
        .tab_width(Length::FillPortion(Tab::iter().count() as u16))
        .padding(5.0)
        .text_size(32.0);
    let content = match state.active_tab {
        Tab::Weather => {
            if let Some(forecast) = &state.forecast {
                view_weather(forecast)
            } else {
                text("Loading...").into()
            }
        }
        Tab::Transit => text("TODO").into(),
    };
    Column::new().push(tabs).push(content).into()
}

/// Generate elements for the weather forecast
fn view_weather(forecast: &Forecast) -> Element<'_, Message> {
    fn view_period(period: &ForecastPeriod) -> Text<'_> {
        text(format!(
            "{} / {}",
            period.temperature(),
            period.prob_of_precip()
        ))
    }

    Column::new()
        .push(view_period(forecast.now()).size(32))
        .extend(
            forecast
                .future_periods()
                .map(|period| view_period(period).into()),
        )
        .into()
}
