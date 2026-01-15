#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod aws;
mod commands;
mod error;
mod util;

use tauri::{App, Manager};
use tauri_plugin_log::{log::LevelFilter, Target, TargetKind};
use time::OffsetDateTime;
use util::dirs::get_user_logs_dir;

use crate::util::formatter::log_time_fmt;

fn initialize(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let _ = app::tray::create_tray(app);

    let window = app.get_webview_window("main").unwrap();

    app::window::set_window_position(&window);
    app::window::disable_transitions(window);

    Ok(())
}

pub fn run() {
    #[cfg(not(rust_analyzer))]
    let context = tauri::generate_context!();

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(tauri_plugin_log::log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(LevelFilter::Info)
                .format(move |out, message, record| {
                    let now = OffsetDateTime::now_utc();
                    let time = now.format(log_time_fmt()).expect("Failed to format time");

                    out.finish(format_args!("{} [{}] {}", time, record.level(), message))
                })
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::Webview),
                    Target::new(TargetKind::Folder {
                        path: get_user_logs_dir(),
                        file_name: Some("els".into()),
                    }),
                ])
                .build(),
        )
        .setup(initialize)
        .invoke_handler(tauri::generate_handler![
            commands::config::load_config,
            commands::config::save_config,
            commands::command::execute_command,
            aws::commands::list_aws_profiles,
            aws::commands::show_aws_config,
            aws::commands::setup_mfa_device,
            aws::commands::connect,
            aws::commands::remove_aws_profile,
            aws::commands::generate_totp_code,
            aws::commands::remove_mfa_device,
            aws::commands::get_profile_names,
            aws::commands::check_mfa_status,
            aws::commands::init_aws_configs,
            aws::commands::show_aws_config,
        ])
        .run(context)
        .expect("error while running tauri application");
}
