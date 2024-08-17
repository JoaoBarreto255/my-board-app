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
    name: &str,
    icon: Option<&str>,
    position: u32,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let icon: Option<String> = icon.and_then(|v| Some(v.to_string()));
    let mut group: Group = Group::new(name.to_string(), icon, position);

    let result = state
        .database_manager
        .lock()
        .unwrap()
        .insert(group.borrow_mut());

    if let Ok(val) = result {
        return match val {
            true => Ok(String::from("Ok")),
            false => Err(String::from("Transaction error!")),
        };
    }

    Err(String::from("Fail to save data!"))
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
