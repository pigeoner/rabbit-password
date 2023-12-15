// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod db;
use db::{
    delete_data, edit_data_by_id, init_db, insert_data, query_data, query_data_by_id, Password,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize)]
struct PwdResult {
    code: i32,
    msg: String,
    data: serde_json::Value,
}

macro_rules! generate_result {
    ($result:expr, $err:expr) => {
        match $result {
            Ok(data) => PwdResult {
                code: 0,
                msg: "success".to_string(),
                data: json!(data),
            },
            Err(err) => PwdResult {
                code: 1,
                msg: format!($err, err),
                data: json!(null),
            },
        }
    };
}

#[tauri::command]
fn query(query_content: Option<String>) -> PwdResult {
    generate_result!(
        query_data(query_content.unwrap_or_else(|| "".to_string())),
        "Failed to fetch data: {:?}"
    )
}

#[tauri::command]
fn query_by_id(id: u32) -> PwdResult {
    generate_result!(query_data_by_id(id), "Failed to fetch data by id: {:?}")
}

#[tauri::command]
fn insert(password: Password) -> PwdResult {
    generate_result!(insert_data(&password), "Failed to insert data: {:?}")
}

#[tauri::command]
fn add_or_edit(id: u32, password: Password) -> PwdResult {
    if id == 0 {
        generate_result!(insert_data(&password), "Failed to insert data: {:?}")
    } else {
        generate_result!(edit_data_by_id(id, &password), "Failed to edit data: {:?}")
    }
}

#[tauri::command]
fn delete(id: u32) -> PwdResult {
    generate_result!(delete_data(id), "Failed to delete data: {:?}")
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            if let Err(err) = init_db() {
                eprintln!("Failed to initialize database: {:?}", err);
                // 数据库初始化失败时，可以选择如何处理，比如退出应用程序
                std::process::exit(1);
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            query,
            insert,
            delete,
            query_by_id,
            add_or_edit
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
