// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use pomodoro_fate_lib::generate_progress_icon;
use std::fs;
use std::time::Duration;
use tauri::async_runtime::spawn;
use tauri::image::Image;
use tauri::{
    generate_handler,
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
};
use tauri::{tray, AppHandle};
use tokio::time::sleep;

#[tauri::command]
fn update_timer(elapsed: u32, total: u32, app_handle: AppHandle) -> tauri::Result<()> {
    log::info!("update_timer called: elapsed={} total={}", elapsed, total);
    let raw = generate_progress_icon(
        32,
        elapsed,
        total,
        [200, 200, 200, 255],
        [50, 150, 250, 255],
    );
    log::info!("raw.len() = {}", raw.len());
    let tmp = std::env::temp_dir().join(format!("pomo_{}.png", elapsed));
    fs::write(&tmp, &raw)?;
    let img = Image::from_path(tmp)?;

    if let Some(tray) = app_handle.tray_by_id("main") {
        tray.set_icon(Some(img))?;
    }
    Ok(())
}

fn start_progress_tray(app_handle: tauri::AppHandle, total_secs: u32) -> tauri::Result<()> {
    spawn(async move {
        for elpased in 0..=total_secs {
            let raw = generate_progress_icon(
                32,
                elpased,
                total_secs,
                [200, 200, 200, 200],
                [50, 150, 250, 255],
            );
            let img = Image::new_owned(raw, 32, 32);
            if let Some(tray) = app_handle.tray_by_id("main") {
                let _ = tray.set_icon(Some(img));
            }
            sleep(Duration::from_secs(1)).await;
        }
    });
    Ok(())
}

fn main() -> tauri::Result<()> {
    tauri::Builder::default()
        .invoke_handler(generate_handler![update_timer])
        .setup(|app| {
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;

            let png_data = generate_progress_icon(32, 0, 1500, [200, 200, 200, 255], [0, 0, 0, 0]);
            let icon_png = Image::new_owned(png_data, 32, 32);
            let title = "残り --:--";

            TrayIconBuilder::new()
                .menu(&menu)
                .title(title)
                .icon(icon_png)
                .build(app)?;

            let handle = app.handle().clone();
            start_progress_tray(handle, 1500)?;
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .run(tauri::generate_context!())
}
