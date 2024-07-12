// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{utils::config::WindowConfig, PhysicalPosition, Position};

const DEBUG: bool = cfg!(debug_assertions);

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = tauri::WindowBuilder::from_config(app, WindowConfig::default())
                .title("Fordingbridge Astronomers Gallery")
                .always_on_top(!DEBUG)
                .fullscreen(!DEBUG)
                .transparent(true)
                .focused(true)
                .build()?;
            window.set_cursor_visible(false)?;

            let monitor = window.current_monitor()?.unwrap();
            let size = monitor.size();
            let pos = Position::Physical(PhysicalPosition {
                y: size.height as i32,
                x: size.width as i32,
            });

            window.set_cursor_position(pos)?;

            return Ok(());
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
