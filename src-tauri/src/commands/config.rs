use crate::util::dirs::get_user_config_file;
use std::{
    fs,
    io::{Read, Write},
    path::PathBuf,
};

const DEFAULT_CONFIG: &str = "{}";

fn write_config(config: &str, path: PathBuf) {
    let parent = path.parent().unwrap();

    fs::create_dir_all(parent).expect("config directory should be writable");

    let mut write_op = fs::File::create(path).unwrap();
    let config_json_value: serde_json::Value = serde_json::from_str(config).unwrap();
    let mut pretty_config = serde_json::to_string_pretty(&config_json_value).unwrap();

    pretty_config.push('\n');

    write_op
        .write_all(pretty_config.as_bytes())
        .expect("config should be writable");
}

#[tauri::command]
pub fn load_config() -> String {
    let config_path = get_user_config_file();

    // Attempt to read the config file
    let read_op = fs::File::open(config_path);
    let mut buffer = String::new();

    match read_op {
        Ok(mut file) => {
            file.read_to_string(&mut buffer)
                .expect("config should be readable");
        }
        Err(_) => {
            write_config(DEFAULT_CONFIG, get_user_config_file());
            buffer = DEFAULT_CONFIG.to_string();
        }
    }

    buffer
}

#[tauri::command]
pub fn save_config(config: &str) {
    write_config(config, get_user_config_file());
}
