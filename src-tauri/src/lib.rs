mod account;
mod email;

use std::sync::Mutex;
use tauri::Manager;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use tracing_appender::rolling::{RollingFileAppender, Rotation};

pub struct AppState {
    pub accounts: Mutex<Vec<account::EmailAccount>>,
}

fn setup_logging() {
    let log_dir = directories::ProjectDirs::from("com", "aithra", "email")
        .map(|dirs| dirs.data_local_dir().to_path_buf())
        .unwrap_or_else(|| std::env::current_dir().unwrap());

    std::fs::create_dir_all(&log_dir).ok();

    let file_appender = RollingFileAppender::new(
        Rotation::DAILY,
        log_dir,
        "aithra.log",
    );

    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(EnvFilter::new("info"))
        .with(fmt::layer().with_writer(non_blocking))
        .with(fmt::layer().with_writer(std::io::stdout))
        .init();

    std::mem::forget(_guard);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    setup_logging();

    tracing::info!("Starting Aithra email client");

    let result = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState {
            accounts: Mutex::new(Vec::new()),
        })
        .invoke_handler(tauri::generate_handler![
            account::add_account,
            account::get_accounts,
            account::delete_account,
            account::test_connection,
            email::get_emails,
            email::get_email,
            email::send_email,
            email::delete_email,
            email::get_folders,
        ])
        .setup(|app| {
            tracing::info!("Application setup complete");
            
            if let Some(window) = app.get_webview_window("main") {
                window.set_title("Aithra Email Client").ok();
            }
            
            Ok(())
        })
        .run(tauri::generate_context!());

    if let Err(e) = result {
        tracing::error!("Error running application: {}", e);
        std::process::exit(1);
    }
}
