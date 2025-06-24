use std::{f32::consts::PI, thread, time::Duration};

use image::{Rgba, RgbaImage};
use tauri::{
    image::Image,
    menu::{Menu, MenuItem},
    tray::{TrayIcon, TrayIconBuilder},
    AppHandle, Runtime,
};

// #[cfg_attr(mobile, tauri::mobile_entry_point)]
// pub fn init_tray<R: Runtime>(app: &AppHandle) -> tauri::Result<()> {
//
//     let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
//     let menu = Menu::with_items(app, &[&quit_i])?;
//     // const ICON: Image<'_> = include_image!("./icons/pomo_fate_icon32.png");
//     let png_data = generate_progress_icon(32,  [100, 180, 240, 255]);
//     let icon_png = Image::new_owned(png_data, 32, 32);
//     let title = "残り --:--";
//
//     let tray: TrayIcon = TrayIconBuilder::new()
//         .menu(&menu)
//         .title(title)
//         .icon(icon_png)
//         .build(app)?;
//
//     log::info!("{}", &title);
//
//     let updater = tray.clone();
//     thread::spawn(move || loop {
//         let (m, s) = get_remaining_time();
//         let label = format!("残り {:02}:{:02}", m, s);
//         updater.set_title(Some(&label)).ok();
//         thread::sleep(Duration::from_secs(1));
//
//         log::info!("{}", &label);
//     });
//
//     Ok(())
// }

// 残り時間を返すダミー関数
pub fn get_remaining_time() -> (u8, u8) {
    (12, 34)
}

pub fn generate_progress_icon(
    size: u32,
    elapsed: u32,
    total: u32,
    color_bg: [u8; 4],
    color_fg: [u8; 4],
) -> Vec<u8> {
    let mut img = RgbaImage::new(size, size);
    let center = (size as f32 / 2.0, size as f32 / 2.0);
    let radius = size as f32 * 0.45;
    let thickness = size as f32 * 0.1;
    let progress = (elapsed as f32 / total as f32).min(1.0);

    // 背景円
    for y in 0..size {
        for x in 0..size {
            let dx = x as f32 - center.0;
            let dy = y as f32 - center.1;
            if (dx * dx + dy * dy).sqrt() <= radius {
                img.put_pixel(x, y, Rgba(color_bg));
            }
        }
    }

    // 進捗リング
    let segments = 360; //360度？
    let end_angle = 2.0 * PI * progress; // 円周の長さ
    for i in 0..((segments as f32) * progress) as u32 {
        let theta = (i as f32) * (2.0 * PI) / (segments as f32) - PI / 2.0;
        // 外周
        let x_out = center.0 + radius * theta.cos();
        let y_out = center.1 + radius * theta.sin();
        // 内周
        let x_in = center.0 + (radius - thickness) * theta.cos();
        let y_in = center.1 + (radius - thickness) * theta.sin();
        // 線分を 1px ずつ埋める
        let steps = (thickness as u32).max(1);
        for t in 0..=steps {
            let lerp = t as f32 / steps as f32;
            let x = x_in + (x_out - x_in) * lerp;
            let y = y_in + (y_out - y_in) * lerp;
            let xi = x.round() as i32;
            let yi = y.round() as i32;
            if (0..size as i32).contains(&xi) && (0..size as i32).contains(&yi) {
                img.put_pixel(xi as u32, yi as u32, Rgba(color_fg));
            }
        }
    }

    img.into_raw()
    // let mut buf = Vec::new();
    // image::codecs::png::PngEncoder::new(&mut buf)
    //     .write_image(&img, size, size, ColorType::Rgba8.into())
    //     .unwrap();
    // buf
}
