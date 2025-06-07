pub mod transit;
pub mod weather;

use crate::Message;
use iced::Task;
use std::{
    sync::LazyLock,
    time::{Duration, Instant},
};

/// Reqwest HTTP client
static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .user_agent("gruber")
        .build()
        .unwrap()
});

/// Trait for a container that fetches and stores data from an external API.
/// This provides methods for fetching the data in a background task and
/// storing it for a certain TTL before expiring and refetching it.
pub trait ExternalData: 'static {
    /// Minimum time between fetching data
    const TTL: Duration;
    type Data: 'static + Send;

    /// Get the current stored data, if any
    fn data(&self) -> Option<&FetchedData<Self::Data>>;

    /// Set the stored data with the output of a fetch
    fn set_data(&mut self, data: FetchedData<Self::Data>);

    /// Pack fetched data into a message so it can be passed from a task back
    /// to the main thread
    fn data_to_message(data: FetchedData<Self::Data>) -> Message;

    /// If the stored data is missing or stale, create a task to refetch it.
    /// Otherwise create an empty task that completes immediately
    fn fetch_if_needed(&self) -> Task<Message> {
        // If the data is missing or stale, refetch
        if self.data().is_none_or(|data| data.is_expired(Self::TTL)) {
            // TODO logging
            Task::future(self.fetch()).and_then(move |data| {
                Task::done(Self::data_to_message(FetchedData::new(data)))
            })
        } else {
            // Data is fresh
            Task::none()
        }
    }

    /// Fetch new data from the external source
    ///
    /// impl Trait return is needed to detach the future's lifetime from the
    /// input
    fn fetch(
        &self,
    ) -> impl 'static + Future<Output = anyhow::Result<Self::Data>> + Send;
}

/// Container for data fetched externally. Includes a timestamp of when it was
/// fetched
#[derive(Debug)]
pub struct FetchedData<T> {
    fetched_at: Instant,
    data: T,
}

impl<T> FetchedData<T> {
    fn new(data: T) -> Self {
        Self {
            fetched_at: Instant::now(),
            data,
        }
    }

    fn is_expired(&self, ttl: Duration) -> bool {
        self.fetched_at + ttl < Instant::now()
    }
}
