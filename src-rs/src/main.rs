// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::utils::config::WindowConfig;

const DEBUG: bool = cfg!(debug_assertions);

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            tauri::WindowBuilder::from_config(app, WindowConfig::default())
                .title("Fordingbridge Astronomers Gallery")
                .always_on_top(!DEBUG)
                .fullscreen(!DEBUG)
                .focused(true)
                .build()?;
            return Ok(());
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
