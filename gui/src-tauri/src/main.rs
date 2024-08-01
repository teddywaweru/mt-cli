// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mt5::{Account, OrderType, Symbols};

#[tauri::command]
fn get_order_types() -> Vec<OrderType> {
    OrderType::list_order_types()
}

#[tauri::command]
fn calculate_order() -> String {
    todo!()
}

#[tauri::command]
fn load_symbols() -> Symbols {
    Symbols::get_symbols()
}

#[tauri::command]
fn update_account_info() -> Account {
    Account::get_account_info()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            update_account_info,
            get_order_types
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
