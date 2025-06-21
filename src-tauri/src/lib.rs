use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    AppHandle, Runtime,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn init_tray<R: Runtime>(app: &AppHandle) -> tauri::Result<()> {
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&quit_i])?;

    TrayIconBuilder::new().menu(&menu).build(app)?;
    Ok(())
}
