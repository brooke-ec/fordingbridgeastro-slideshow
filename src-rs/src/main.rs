// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use specta::Type;
use std::{collections::VecDeque, env, process::Command};
use tauri::utils::config::WindowConfig;
use tauri::{generate_handler, Manager, Result, Runtime, State, Window};

#[derive(Serialize, Type, PartialEq, Eq)]
enum Mode {
    SlideShow,
    ScreenSaver,
    Configuration,
}

#[derive(Serialize, Type)]
struct Configuration {
    mode: Mode,
}

#[tauri::command]
#[specta::specta]
fn get_configuration(cfg: State<Configuration>) -> &Configuration {
    return cfg.inner();
}

#[tauri::command]
#[specta::specta]
fn start_slideshow() {
    Command::new(env::current_exe().unwrap())
        .arg("/r")
        .spawn()
        .unwrap();
}

#[tauri::command]
#[specta::specta]
fn install_screensaver() {
    Command::new("rundll32.exe")
        .arg("desk.cpl,InstallScreenSaver")
        .args(
            env::current_exe()
                .unwrap()
                .to_string_lossy()
                .split_whitespace(),
        )
        .spawn()
        .unwrap();
}

fn main() {
    // See https://ftp.zx.net.nz/pub/archive/ftp.microsoft.com/MISC/KB/en-us/182/383.HTM#:~:text=ScreenSaver%20%2D%20Show%20the%20Settings%20dialog,s%20%2D%20Run%20the%20Screen%20Saver.
    let mut args: VecDeque<String> = env::args().collect();
    args.pop_front();

    let cfg = Configuration {
        mode: match args.front() {
            None => Mode::Configuration,
            Some(a) => match a.as_str() {
                "/S" => Mode::Configuration,
                "/r" => Mode::SlideShow,
                "/s" => Mode::ScreenSaver,
                s => match s.starts_with("/c") {
                    true => Mode::Configuration,
                    false => panic!("Mode not implemented"),
                },
            },
        },
    };

    #[cfg(debug_assertions)]
    tauri_specta::ts::export(
        specta::collect_types![get_configuration, start_slideshow, install_screensaver],
        "../src/lib/specta.ts",
    )
    .unwrap();

    tauri::Builder::default()
        .setup(|app| {
            let cfg = app.try_state::<Configuration>().unwrap();

            let main = window(app, &cfg)?;

            let monitors = main.available_monitors()?;
            for i in 0..monitors.len() {
                let monitor = &monitors[i];
                let window = match i {
                    0 => &main,
                    _ => &window(app, &cfg)?,
                };

                let pos = monitor.position();
                window.set_position(tauri::PhysicalPosition { x: pos.x, y: 0 })?;
                window.center()?;
            }

            // Workaround for https://github.com/tauri-apps/tauri/issues/10231
            #[cfg(not(debug_assertions))]
            if cfg.mode != Mode::Configuration {
                use tauri::{PhysicalPosition, Position};

                let monitor = main.current_monitor()?.unwrap();
                let size = monitor.size();
                let pos = Position::Physical(PhysicalPosition {
                    y: size.height as i32,
                    x: size.width as i32,
                });

                main.set_cursor_position(pos)?;
            }

            return Ok(());
        })
        .manage(cfg)
        .invoke_handler(generate_handler![
            get_configuration,
            start_slideshow,
            install_screensaver
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn window<'a, M, R>(app: &'a M, cfg: &Configuration) -> Result<Window<R>>
where
    M: Manager<R>,
    R: Runtime,
{
    let mut c = WindowConfig::default();
    c.label = uuid::Uuid::new_v4().simple().to_string();

    c.title = "Fordingbridge Astronomers Gallery".to_owned();
    c.always_on_top = cfg.mode == Mode::ScreenSaver;
    c.resizable = cfg.mode != Mode::Configuration;
    c.transparent = true;
    c.focus = true;

    #[cfg(not(debug_assertions))]
    {
        c.fullscreen = cfg.mode != Mode::Configuration;
    }

    #[cfg(debug_assertions)]
    {
        c.fullscreen = false;
    }

    return Ok(tauri::WindowBuilder::from_config(app, c).build()?);
}
