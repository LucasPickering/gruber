pub mod transit;
pub mod weather;

use crate::{Message, config::Config};
use anyhow::{Context, anyhow};
use iced::Task;
use log::{error, info, warn};
use serde::de::DeserializeOwned;
use std::{
    sync::{Arc, LazyLock, RwLock},
    thread,
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
    fn fetch_if_needed(&self, config: &Config) -> Task<Message> {
        // If the data is missing or stale, refetch
        if self.data().is_none_or(|data| data.is_expired(Self::TTL)) {
            Task::future(Self::fetch(config)).and_then(move |data| {
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
        config: &Config,
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

/// Helper for regularly fetching data from an API
#[derive(Debug)]
pub struct ApiFetcher<T> {
    url: String,
    /// Frequency at which to refetch data
    ttl: Duration,
    /// Data loaded from the API. The load is done in a separate thread and
    /// deposited here
    data: Arc<RwLock<Option<(T, Instant)>>>,
}

impl<T> ApiFetcher<T>
where
    T: 'static + Clone + DeserializeOwned + Send + Sync,
{
    pub fn new(url: String, ttl: Duration) -> Self {
        Self {
            url,
            ttl,
            data: Default::default(),
        }
    }

    /// Get the latest data. If the data is missing or outdated, spawn a task to
    /// re-fetch it
    pub fn data(&self) -> Option<T> {
        let Some(guard) = self.data.try_read().ok() else {
            // Content is so low that we don't ever expect to hit this
            warn!("Failed to grab data read lock");
            return None;
        };

        if let Some((data, fetched_at)) = guard.as_ref() {
            // If forecast is stale, fetch a new one in the background
            if *fetched_at + self.ttl < Instant::now() {
                self.fetch_latest();
            }

            // Return the data even if it's old. Old is better than nothing.
            // Clone so we can release the lock
            Some(data.clone())
        } else {
            self.fetch_latest();
            None
        }
    }

    /// Spawn a task to fetch the latest forecase in the background
    fn fetch_latest(&self) {
        let lock = Arc::clone(&self.data);
        let request = ureq::get(&self.url);

        thread::spawn(move || {
            let url = request.url().to_owned();
            // Shitty try block
            let result: anyhow::Result<()> = (|| {
                info!("Fetching new data from {url}");
                let response = request.call().with_context(|| {
                    format!("Error fetching data from {url}")
                })?;
                let data: T = response.into_json().with_context(|| {
                    format!("Error parsing data from {url} as JSON")
                })?;

                // Stringify the error to dump the lifetime
                *lock.write().map_err(|err| anyhow!("{err}"))? =
                    Some((data, Instant::now()));
                Ok(())
            })();

            if let Err(err) = result {
                error!("Error fetching data from {url}: {err:?}")
            }
        });
    }
}
