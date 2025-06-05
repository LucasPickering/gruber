use anyhow::Context;
use log::info;
use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub window_size: (f32, f32),
    pub forecast_office: String,
    pub forecast_gridpoint: (u32, u32),
    // pub transit_lines: Vec<TransitLine>,
}

impl Config {
    const PATH: &'static str = "./config.json";

    /// Load config from file
    pub fn load() -> anyhow::Result<Self> {
        info!("Loading config from `{}`", Self::PATH);
        let file = File::open(Self::PATH)?;
        serde_json::from_reader(file)
            .context(format!("Error parsing config file {}", Self::PATH))
    }
}
