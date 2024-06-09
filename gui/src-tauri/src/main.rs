// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mt5::{Account,Symbols, OrderType, OrderRequest};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command(rename_all = "snake_case")]
fn update_account_info() -> Account {
    Account::default()
}

#[tauri::command(rename_all = "snake_case")]
fn get_active_trades() {}

#[tauri::command(rename_all = "snake_case")]
fn get_symbols() -> Symbols {
    Symbols::default()
}
#[tauri::command(rename_all = "snake_case")]
fn get_order_types() -> Vec<OrderType> {
    OrderType::list_order_types()
}

#[tauri::command(rename_all = "snake_case")]
fn get_instant_rates() {}

#[tauri::command(rename_all = "snake_case")]
fn calculate_order() -> OrderRequest {
    todo!()

}
#[tauri::command(rename_all = "snake_case")]
fn execute_instant_order(order: String) {
    let risk = 0.02;

    // let response = Mt5Bridge::generate_order(&symbol, order_type, risk);
    todo!()
}

#[tauri::command(rename_all = "snake_case")]
fn execute_pending_order() {}

#[tauri::command(rename_all = "snake_case")]
fn track_prices() {}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            update_account_info,
            get_symbols,
            get_order_types
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
