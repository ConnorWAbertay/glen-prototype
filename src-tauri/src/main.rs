#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod sql_handler;
use sql_handler::{AssetInfo, UpdateInfo};
use tauri::Window;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn assetList() -> Vec<AssetInfo> {
    println!("ASSETLIST INVOKED");
    return sql_handler::query_assets("1");
}

#[tauri::command]
fn update(info : UpdateInfo) {
    println!("HEY");
    sql_handler::update_asset(info);
}

#[tauri::command]
fn error(message : &str) {
    println!("{}",message);
}

#[tauri::command]
fn window_loaded(w: Window){
  w.emit("window_loaded", "").unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![assetList, error, window_loaded, update, sql_handler::get_name, sql_handler::insert_manu,
            sql_handler::insert_model, sql_handler::insert_type, sql_handler::insert_asset])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
