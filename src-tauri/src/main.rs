mod config;
mod util;
mod weather;

use crate::config::Config;
use tauri_plugin_window_state::StateFlags;

fn main() -> anyhow::Result<()> {
    let config = Config::load()?;

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Info)
                .level_for("gruber::*", log::LevelFilter::Trace)
                .build(),
        )
        .plugin(
            tauri_plugin_window_state::Builder::new()
                .with_state_flags(StateFlags::POSITION)
                .build(),
        )
        .manage(config)
        .invoke_handler(tauri::generate_handler![weather::fetch_weather])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
