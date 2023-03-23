// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, path::PathBuf, str::FromStr};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn read_markdown_source(path: &str) -> String {
    if let Ok(p) = PathBuf::from_str(path) {
        if p.exists() && p.is_file() {
            return fs::read_to_string(&p).unwrap();
        }
    }
    return String::new();
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            match app.get_cli_matches() {
                Ok(matches) => {
                    println!("{:?}", matches);
                }
                Err(_) => {}
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, read_markdown_source])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
