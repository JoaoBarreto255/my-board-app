// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;

use std::borrow::{Borrow, BorrowMut};
use std::rc::Rc;
use std::sync::Mutex;

use database::fixtures::create_tables;
use database::manager::Manager;
use database::models::Group;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn create_group(
    // name: &str,
    // icon: Option<&str>,
    // position: u32,
    mut new_group: Group,
    app_state: tauri::State<'_, AppState>,
) -> Result<Group, String> {
    app_state.database_manager
        .lock()
        .unwrap()
        .insert(new_group.borrow_mut())
        .is_ok()
        .then_some(new_group)
        .ok_or(String::from("Transaction error!"))
}

struct AppState {
    database_manager: Mutex<Manager>,
}

fn main() {
    let connection =
        Manager::new(String::from("./.my-board-db.db")).expect("Could not open database");
    let state = AppState {
        database_manager: Mutex::new(connection),
    };

    tauri::Builder::default()
        .setup(|_app| {
            create_tables().expect("Error trying create database!\n:-(");

            return Ok(());
        })
        .manage(state)
        .invoke_handler(tauri::generate_handler![greet, create_group])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
