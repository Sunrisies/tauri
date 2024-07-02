use open::that;
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
  };

#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn opengug(name: &str) -> String {
    that(name).expect("Failed to open browser");
    format!("Hello, {}! You've been greeted from Rust!", name)

    // that("https://cn.vitejs.dev/").expect("Failed to open browser");
}

mod tray;
fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, opengug,open_about])
        .system_tray(tray::menu()) //✅ 将 `tauri.conf.json` 上配置的图标添加到系统托盘
        .on_system_tray_event(|app, event| tray::handler(app, event)) // ✅ 注册系统托盘事件处理程序
        .run(context)
        .expect("error while running tauri application");
}
#[tauri::command]
async fn open_about(handle:tauri::AppHandle){
    tauri::WindowBuilder::new(
        &handle,
        "search",
        tauri::WindowUrl::App("/search".into())
        // tauri::WindowUrl::App("/search".into().unwrap()),
    )
    .title("search")
    .inner_size(400.0, 300.0).center()
    .build()
    .unwrap();
}
// async fn create_window(label: &str, app: &tauri::AppHandle) {
//     let config = get_window_config(label);
//     if let Some(window) = app.get_window(label) {
//         let _ = window.show();
//         let _ = window.set_focus();
//         let _ = window.request_user_attention(Some(UserAttentionType::Informational));
//     } else {
//         let window = tauri::window::WindowBuilder::new(
//             app,
//             label,
//             tauri::WindowUrl::App(label.parse().unwrap()),
//         )
//         .center()
//         .title(config["title"].as_str().unwrap())
//         .inner_size(config["width"].as_f64().unwrap(), config["height"].as_f64().unwrap())
//         .build()
//         .unwrap();
//         window.set_focus();
//     }
// }
