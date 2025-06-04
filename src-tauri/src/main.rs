use tauri_plugin_window_state::StateFlags;

fn main() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_window_state::Builder::new()
                .with_state_flags(StateFlags::POSITION)
                .build(),
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
