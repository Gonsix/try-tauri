// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use serde::{Deserialize, Serialize};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn simple_hello() {
    println!("I was invoked from JS!")
}

#[tauri::command]
fn hello_with_message(message: String) -> String {
    println!("Hello, {message}");
    format!("Hello, {message}")
}

#[derive(Debug, Serialize, Deserialize)]
struct MyMessage {
    field_str: String,
    field_int: i32,
}

#[tauri::command]
fn hello_with_object(message: MyMessage) -> MyMessage {
    let str1 = message.field_str;
    let int1 = message.field_int;

    let str2 = str1.to_uppercase();
    let int2 = int1 * 2;

    let result = MyMessage {
        field_str: str2,
        field_int: int2,
    };

    println!("{result:#?}");
    result
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            simple_hello,
            hello_with_message,
            hello_with_object
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
