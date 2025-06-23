// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use pomodoro_fate_lib::init_tray;

fn main() -> tauri::Result<()> {
    tauri::Builder::default()
        .setup(|app| {
            init_tray::<tauri::Wry>(&app.handle())?;
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .run(tauri::generate_context!())
}
