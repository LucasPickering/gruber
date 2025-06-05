use serde::Serialize;
use std::sync::LazyLock;

pub static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .user_agent("gruber")
        .build()
        .unwrap()
});

/// Wrapper to serialize an error as a string, so it can be passed to JS
#[derive(Debug)]
pub struct SerdeError(anyhow::Error);

impl From<anyhow::Error> for SerdeError {
    fn from(error: anyhow::Error) -> Self {
        SerdeError(error)
    }
}

impl Serialize for SerdeError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        format!("{:#}", self.0).serialize(serializer)
    }
}
