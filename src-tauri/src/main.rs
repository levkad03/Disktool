// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use disktool::scan_dir;
use std::path::PathBuf;

#[tauri::command]
fn scan(path: String, top: usize) -> disktool::DirectoryReport {
    scan_dir(&PathBuf::from(path), top)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![scan])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
