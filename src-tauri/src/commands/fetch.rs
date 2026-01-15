use std::fs;

use crate::util::config;

#[tauri::command]
pub async fn fetch_commands() -> String {
    config::verify();

    let path = config::akaw_path()
        .join("commands")
        .join("commands.json");

    match fs::read_to_string(path) {
        Ok(json) => json,
        Err(_) => "{}".to_string(),
    }
}
