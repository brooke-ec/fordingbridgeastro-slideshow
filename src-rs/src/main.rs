// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use specta::Type;
use std::{collections::VecDeque, env};
use tauri::{generate_handler, utils::config::WindowConfig, State};

#[derive(Serialize, Type, PartialEq, Eq, Clone, Copy)]
enum Mode {
    SlideShow,
    ScreenSaver,
}

#[derive(Serialize, Type, Clone, Copy)]
struct Configuration {
    mode: Mode,
}

#[tauri::command]
#[specta::specta]
fn get_configuration(configuration: State<Configuration>) -> &Configuration {
    return configuration.inner();
}

fn main() {
    // See https://ftp.zx.net.nz/pub/archive/ftp.microsoft.com/MISC/KB/en-us/182/383.HTM#:~:text=ScreenSaver%20%2D%20Show%20the%20Settings%20dialog,s%20%2D%20Run%20the%20Screen%20Saver.
    let mut args: VecDeque<String> = env::args().collect();
    args.pop_front();

    let configuration = Configuration {
        mode: match args.front() {
            None => Mode::SlideShow,
            Some(a) => match a.as_str() {
                "/s" => Mode::ScreenSaver,
                #[cfg(not(debug_assertions))]
                _ => todo!(),
                #[cfg(debug_assertions)]
                _ => Mode::SlideShow,
            },
        },
    };

    #[cfg(debug_assertions)]
    tauri_specta::ts::export(
        specta::collect_types![get_configuration],
        "../src/lib/specta.ts",
    )
    .unwrap();

    tauri::Builder::default()
        .setup(move |app| {
            let _window = tauri::WindowBuilder::from_config(app, WindowConfig::default())
                .title("Fordingbridge Astronomers Gallery")
                .always_on_top(configuration.mode == Mode::ScreenSaver)
                .fullscreen(cfg!(not(debug_assertions)))
                .transparent(true)
                .focused(true)
                .build()?;

            // Workaround for https://github.com/tauri-apps/tauri/issues/10231
            #[cfg(not(debug_assertions))]
            {
                use tauri::{PhysicalPosition, Position};

                let monitor = _window.current_monitor()?.unwrap();
                let size = monitor.size();
                let pos = Position::Physical(PhysicalPosition {
                    y: size.height as i32,
                    x: size.width as i32,
                });

                _window.set_cursor_position(pos)?;
            }

            return Ok(());
        })
        .manage(configuration)
        .invoke_handler(generate_handler![get_configuration])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
