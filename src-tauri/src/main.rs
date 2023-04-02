// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, path::PathBuf, str::FromStr};
use glob::glob;

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

// This doesn't work every single time. Find out why
#[tauri::command]
fn get_matching_paths(path: &str) -> String {
    let mut output = Vec::new();
    if let Ok(paths) = glob(&format!("{}*", path)) {
        for entry in paths {
            match entry {
                Ok(p) => output.push(String::from(p.to_str().unwrap())),
                Err(_) => {},
            }
        }
    }
    return output.join("\n");
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
        .invoke_handler(tauri::generate_handler![greet, read_markdown_source, get_matching_paths])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
