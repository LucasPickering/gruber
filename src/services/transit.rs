use crate::{
    Message,
    config::Config,
    services::{CLIENT, ExternalData, FetchedData},
};
use anyhow::Context;
use chrono::{DateTime, Utc};
use indexmap::IndexMap;
use itertools::Itertools;
use log::{error, info};
use serde::Deserialize;
use std::{fmt::Display, time::Duration};

#[derive(Debug)]
pub struct Transit {
    url: String,
    lines: Vec<TransitLine>,
    /// Prediction data loaded from the API
    data: Option<FetchedData<ApiPredictions>>,
}

impl Transit {
    /// Max number of pending departures to show for a stop
    const MAX_PREDICTIONS: usize = 3;

    pub fn new(config: &Config) -> Self {
        let all_stops = config
            .transit_lines
            .iter()
            .flat_map(|line| &line.stops)
            .map(|stop| stop.id);
        let url = format!(
            "https://api-v3.mbta.com/predictions?filter[stop]={}",
            all_stops.format(",")
        );
        Self {
            url,
            lines: config.transit_lines.clone(),
            data: None,
        }
    }

    /// Get predictions for all stops on all lines
    pub fn predictions(&self) -> Predictions {
        // Group predictions as (line, stops)
        let mut grouped: IndexMap<String, Vec<StopPrediction>> = self
            .lines
            .iter()
            .map(|line| {
                (
                    line.name.clone(),
                    line.stops
                        .iter()
                        .map(|stop| StopPrediction {
                            id: stop.id,
                            name: stop.name.clone(),
                            predictions: CountdownList::default(),
                        })
                        .collect(),
                )
            })
            .collect();

        // Pull data from the most recent response
        if let Some(data) = &self.data {
            for prediction in &data.data.data {
                // Departure time will be empty if the stop is being skipped
                let Some(departure_time) = prediction.attributes.departure_time
                else {
                    continue;
                };
                let route_id = &prediction.relationships.route.data.id;
                let Some(stops) = grouped.get_mut(route_id) else {
                    error!("Unknown route {route_id}");
                    continue;
                };
                let stop_id = &prediction.relationships.stop.data.id;

                let Some(stop_prediction) = stops
                    .iter_mut()
                    .find(|stop| &stop.id.to_string() == stop_id)
                else {
                    error!("Unknown stop {stop_id} for route {route_id}");
                    continue;
                };
                stop_prediction.predictions.push(departure_time);
            }
        }

        // We want to show empty data if we don't have an API response yet
        let lines = grouped
            .into_iter()
            .map(|(name, stops)| LinePrediction { name, stops })
            .collect();
        Predictions { lines }
    }
}

impl ExternalData for Transit {
    const TTL: Duration = Duration::from_secs(30);
    type Data = ApiPredictions;

    fn data(&self) -> Option<&FetchedData<Self::Data>> {
        self.data.as_ref()
    }

    fn set_data(&mut self, data: FetchedData<Self::Data>) {
        self.data = Some(data);
    }

    fn data_to_message(data: FetchedData<Self::Data>) -> Message {
        Message::TransitFetched(data)
    }

    fn fetch(
        &self,
    ) -> impl 'static + Future<Output = anyhow::Result<Self::Data>> + Send {
        info!("Fetching transit data from {}", self.url);
        let request = CLIENT.get(&self.url);
        async move {
            let response =
                request.send().await.context("Error fetching transit")?;
            response
                .error_for_status()?
                .json()
                .await
                .context("Error parsing transit")
        }
    }
}

/// Configuration for a transit line to show predictions for
#[derive(Clone, Debug, Deserialize)]
pub struct TransitLine {
    pub name: String,
    /// Stops on the line to monitor
    pub stops: Vec<TransitStop>,
}

/// Configuration for a single stop on a transit line
#[derive(Clone, Debug, Deserialize)]
pub struct TransitStop {
    /// Display name for the stop/direction
    pub name: String,
    /// API ID of the stop
    pub id: u32,
}

#[derive(Debug)]
pub struct Predictions {
    pub lines: Vec<LinePrediction>,
}

#[derive(Debug)]
pub struct LinePrediction {
    pub name: String,
    pub stops: Vec<StopPrediction>,
}

#[derive(Debug)]
pub struct StopPrediction {
    pub id: u32,
    pub name: String,
    pub predictions: CountdownList,
}

#[derive(Debug, Default)]
pub struct CountdownList(Vec<Countdown>);

impl CountdownList {
    /// Add a timestamp to the list of countdowns. If already at max size, throw
    /// it away
    fn push(&mut self, departure_time: DateTime<Utc>) {
        if self.0.len() < Transit::MAX_PREDICTIONS {
            let now = Utc::now();
            let countdown = Countdown((departure_time - now).num_minutes());
            self.0.push(countdown);
        }
    }
}

impl Display for CountdownList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Format countdowns as "1m, 5m, 10m"
        write!(
            f,
            "{}",
            self.0
                .iter()
                .format_with(", ", |countdown, f| f(&format_args!(
                    "{}m",
                    countdown.0
                )))
        )
    }
}

/// Number of minutes until an event
#[derive(Debug)]
pub struct Countdown(i64);

impl Display for Countdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// <https://api-v3.mbta.com/docs/swagger/index.html#/Prediction/ApiWeb_PredictionController_index>
#[derive(Clone, Debug, Deserialize)]
pub struct ApiPredictions {
    data: Vec<Prediction>,
}

#[derive(Clone, Debug, Deserialize)]
struct Prediction {
    attributes: Attributes,
    relationships: Relationships,
}

#[derive(Clone, Debug, Deserialize)]
struct Attributes {
    departure_time: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Deserialize)]
struct Relationships {
    route: Relationship,
    stop: Relationship,
}

#[derive(Clone, Debug, Deserialize)]
struct Relationship {
    data: RelationshipData,
}

#[derive(Clone, Debug, Deserialize)]
struct RelationshipData {
    id: String,
}
