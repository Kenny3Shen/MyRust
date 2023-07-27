// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn my_custom_command(a: f32, b: f32) -> f32 {
    let c= a * b;
    c
}

#[tauri::command]
fn my_custom_command2(a: f32, b: f32) -> f32 {
    let c= a + b;
    c
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, my_custom_command, my_custom_command2])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
