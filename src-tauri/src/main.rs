// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod keyboard_listener;
use std::thread;
use tauri::{ CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem };

#[derive(Clone, serde::Serialize)]
struct Payload {
    mode: String,
    message: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    // 创建系统托盘实现
    let show = CustomMenuItem::new("show_window".to_string(), "Show Window");
    let hide = CustomMenuItem::new("hide_window".to_string(), "Hide Window");
    let exit = CustomMenuItem::new("exit".to_string(), "Exit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(exit);

    let system_tary = SystemTray::new().with_menu(tray_menu);

    // 创建应用窗口
    tauri::Builder::default()
        // 设置系统托盘
        .system_tray(system_tary)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick {   id, .. } => match id.as_str() {
                "show_window" => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                }
                "hide_window" => {
                    let window = app.get_window("main").unwrap();
                    window.hide().unwrap();
                },
                "exit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![greet])
        .setup(move |app| {
            let wv = app.get_window("main").unwrap();
            wv.set_always_on_top(true).unwrap();

            thread::spawn(move || {
                keyboard_listener::run_listener(move |s: &str, s1: &str| {
                    if let Err(err) = wv.emit(
                        "keypress",
                        Payload {
                            mode: String::from(s),
                            message: String::from(s1),
                        },
                    ) {
                        eprintln!("Error while emitting event: {:?}", err);
                    }
                })
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
