// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::WindowEvent;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        /* .on_window_event(|w, e| {
            if let WindowEvent::Resized(_) = e {
                std::thread::sleep(std::time::Duration::from_nanos(1));
            }
        }) */
        .setup(|app| {
            let s = Some("string".to_string());
            let s2 = s.clone();
            dbg!(&app.config().bundle.license);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
