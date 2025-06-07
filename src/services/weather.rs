use crate::{config::Config, services::CLIENT};
use anyhow::Context;
use chrono::{DateTime, Local, NaiveTime, Utc};
use log::info;
use serde::Deserialize;
use std::time::Duration;

const FORECAST_TTL: Duration = Duration::from_secs(60);
const API_HOST: &str = "https://api.weather.gov";
// Start and end (inclusive) of forecast times that *should* be shown.
// unstable: const unwrap https://github.com/rust-lang/rust/issues/67441
const DAY_START: NaiveTime = NaiveTime::from_hms_opt(4, 30, 0).unwrap();
const DAY_END: NaiveTime = NaiveTime::from_hms_opt(22, 30, 0).unwrap();
/// We show every n periods in the future
const PERIOD_INTERNAL: usize = 4;

/// Fetch weather forecast data
///
/// impl Trait return is needed to detach the future's lifetime from the input
pub fn fetch_weather(
    config: &Config,
) -> impl 'static + Future<Output = anyhow::Result<Forecast>> {
    let url = format!(
        "{}/gridpoints/{}/{},{}/forecast/hourly",
        API_HOST,
        config.forecast_office,
        config.forecast_gridpoint.0,
        config.forecast_gridpoint.1
    );
    async {
        info!("Fetching weather data from {}", url);
        let response = CLIENT
            .get(url)
            .send()
            .await
            .context("Error fetching weather")?;
        response
            .error_for_status()?
            .json()
            .await
            .context("Error parsing weather")
    }
}

///https://www.weather.gov/documentation/services-web-api#/default/gridpoint_forecast
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Forecast {
    properties: ForecastProperties,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForecastProperties {
    periods: Vec<ForecastPeriod>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForecastPeriod {
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    temperature: i32,
    probability_of_precipitation: Unit,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Unit {
    pub value: Option<i32>,
}

impl Forecast {
    /// Get the current forecast period
    pub fn now(&self) -> &ForecastPeriod {
        &self.properties.periods[0]
    }

    /// Get the list of periods that should be shown in the list. This skips
    /// periods in the middle of the night.
    pub fn future_periods(&self) -> impl '_ + Iterator<Item = &ForecastPeriod> {
        let day_range = DAY_START..=DAY_END;
        self.properties
            .periods
            .iter()
            .skip(1)
            .step_by(PERIOD_INTERNAL)
            .filter(move |period| {
                day_range.contains(&period.start_time().time())
            })
    }
}

impl ForecastPeriod {
    /// Localized timestamp for the start of this period
    pub fn start_time(&self) -> DateTime<Local> {
        self.start_time.with_timezone(&Local)
    }

    /// Formatted temperature
    pub fn temperature(&self) -> String {
        format!("{:.0}°", self.temperature)
    }

    /// Formatted probability of precipitation
    pub fn prob_of_precip(&self) -> String {
        format!(
            "{:.0}%",
            self.probability_of_precipitation.value.unwrap_or_default()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn period(
        time: &str,
        hours: i64,
        temperature: i32,
        probability_of_precipitation: i32,
    ) -> ForecastPeriod {
        let start_time = time.parse().unwrap();
        let end_time = start_time + chrono::Duration::hours(hours);
        ForecastPeriod {
            start_time,
            end_time,
            temperature,
            probability_of_precipitation: Unit {
                value: Some(probability_of_precipitation),
            },
        }
    }

    #[test]
    fn test_now() {
        let forecast = Forecast {
            properties: ForecastProperties {
                periods: vec![
                    period("2024-05-24T17:00:00Z", 1, 84, 1),
                    period("2024-05-24T18:00:00Z", 1, 85, 0),
                    period("2024-05-24T19:00:00Z", 1, 86, 0),
                ],
            },
        };

        assert_eq!(forecast.now(), &period("2024-05-24T17:00:00Z", 1, 84, 1));
    }
}
