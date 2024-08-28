// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;

use std::borrow::{Borrow, BorrowMut};
use std::sync::Mutex;

use database::fixtures::create_tables;
use database::manager::Manager;
use database::models::{Board, Group, ModelQueryBuilder, State, Task};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn create_entity<T: ModelQueryBuilder>(
    mut new_entity: T,
    app_state: tauri::State<'_, AppState>,
) -> Result<T, String> {
    app_state
        .database_manager
        .lock()
        .unwrap()
        .insert(new_entity.borrow_mut())
        .is_ok()
        .then_some(new_entity)
        .ok_or(String::from("Transaction error!"))
}

fn update_entity<T: ModelQueryBuilder>(
    entity: T,
    app_state: tauri::State<'_, AppState>,
) -> Result<T, String> {
    app_state
        .database_manager
        .lock()
        .unwrap()
        .update(entity.borrow())
        .is_ok()
        .then_some(entity)
        .ok_or(String::from("Transaction error!"))
}

#[tauri::command]
fn create_group(new_group: Group, app_state: tauri::State<'_, AppState>) -> Result<Group, String> {
    create_entity(new_group, app_state)
}

#[tauri::command]
fn update_group(group: Group, app_state: tauri::State<'_, AppState>) -> Result<Group, String> {
    update_entity(group, app_state)
}

// #[tauri::command]
// fn delete_group(
//     mut group: Group,
//     app_state: tauri::State<'_, AppState>,
// ) -> Result<bool, String> {
//     app_state.database_manager
//         .lock()
//         .unwrap()
//         .delete(group.borrow())
//         .and_then(|x| Ok(x))
// }

#[tauri::command]
fn create_board(new_board: Board, app_state: tauri::State<'_, AppState>) -> Result<Board, String> {
    create_entity(new_board, app_state)
}

#[tauri::command]
fn update_board(board: Board, app_state: tauri::State<'_, AppState>) -> Result<Board, String> {
    update_entity(board, app_state)
}

#[tauri::command]
fn create_state(new_state: State, app_state: tauri::State<'_, AppState>) -> Result<State, String> {
    create_entity(new_state, app_state)
}

#[tauri::command]
fn update_state(state: State, app_state: tauri::State<'_, AppState>) -> Result<State, String> {
    update_entity(state, app_state)
}

#[tauri::command]
fn create_task(task: Task, app_state: tauri::State<'_, AppState>) -> Result<Task, String> {
    create_entity(task, app_state)
}

#[tauri::command]
fn update_task(task: Task, app_state: tauri::State<'_, AppState>) -> Result<Task, String> {
    update_entity(task, app_state)
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
        .invoke_handler(tauri::generate_handler![
            greet,
            create_group,
            update_group,
            create_board,
            update_board,
            create_state,
            update_state,
            create_task,
            update_task
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
