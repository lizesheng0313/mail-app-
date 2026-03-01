// 模块声明
mod commands;
mod mail;

use commands::{add_external_mailbox, fetch_emails, is_tauri, test_connection};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            add_external_mailbox,
            fetch_emails,
            test_connection,
            is_tauri,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

