use crate::{Message, State, Tab, services::weather::Forecast};
use iced::{
    Element, Length, Padding,
    alignment::Horizontal,
    widget::{Column, Container, text},
};
use iced_aw::{Grid, TabBar, TabLabel, grid_row};

const FONT_SIZE_MEDIUM: f32 = 32.0;
const FONT_SIZE_LARGE: f32 = 48.0;

/// Generate display elements
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
        .text_size(FONT_SIZE_MEDIUM);
    let content = match state.active_tab {
        Tab::Weather => {
            if let Some(forecast) = state.weather.forecast() {
                view_weather(forecast)
            } else {
                text("Loading...").into()
            }
        }
        Tab::Transit => text("TODO").into(),
    };
    Column::new()
        .push(tabs)
        .push(Container::new(content).padding(16.0))
        .into()
}

/// Generate elements for the weather forecast
fn view_weather(forecast: &Forecast) -> Element<'_, Message> {
    // Now
    let now = forecast.now();
    let now_text =
        text(format!("{} / {}", now.temperature(), now.prob_of_precip()))
            .size(FONT_SIZE_LARGE);

    // Later
    let num_future_periods = 8;
    let future_grid = Grid::with_rows(
        forecast
            .future_periods()
            .take(num_future_periods)
            .map(|period| {
                grid_row!(
                    text(format!("{}", period.start_time().format("%_I%P"))),
                    text(period.temperature()),
                    text(period.prob_of_precip()),
                )
            })
            .collect(),
    )
    .padding(Padding::ZERO.top(8.0))
    .horizontal_alignment(Horizontal::Right)
    .column_spacing(8.0);

    Column::new().push(now_text).push(future_grid).into()
}
