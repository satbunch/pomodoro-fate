use std::{thread, time::Duration};

use tauri::{
    image::Image,
    include_image,
    menu::{Menu, MenuItem},
    tray::{TrayIcon, TrayIconBuilder},
    AppHandle, Runtime,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn init_tray<R: Runtime>(app: &AppHandle) -> tauri::Result<()> {
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&quit_i])?;
    const ICON: Image<'_> = include_image!("./icons/pomo_fate_icon32.png");

    let tray: TrayIcon = TrayIconBuilder::new().menu(&menu).icon(ICON).build(app)?;
    Ok(())
}

// 残り時間を返すダミー関数
pub fn get_remaining_time() -> (u8, u8) {
    (12, 34)
}

// TrayHandleを受け取って定期更新する関数
pub fn spawn_tray_updater(tray: TrayIcon) {
    thread::spawn(move || loop {
        let (m, s) = get_remaining_time();
        let text = format!("残り {:02}:{:02}", m, s);
        thread::sleep(Duration::from_secs(1));
    });
}
