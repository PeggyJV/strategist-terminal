// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use steward::refresh_steward_versions_thread;

use tauri_plugin_log::LogTarget;

mod adaptors;
mod app;
mod cellar_call;
mod commands;
mod lifecycle;
mod logging;
mod schedule;
mod sommelier;
mod state;
mod steward;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        // Initialize state
        .manage(state::Sommelier::new())
        .manage(state::Requests::new())
        .manage(state::Stewards::new())
        .plugin(
            // Configure logging
            tauri_plugin_log::Builder::default()
                .format(logging::format_log)
                .targets([LogTarget::Stdout])
                .level(log::LevelFilter::Info)
                .level_for("app", log::LevelFilter::Trace)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            commands::schedule_request,
            commands::steward_versions,
            commands::configure,
            commands::request_state,
            commands::remove_request,
            commands::clear_requests,
        ])
        // Run background threads
        .setup(|app| {
            let app_handle = app.handle();

            // Monitor subscribers' Steward versions
            tauri::async_runtime::spawn(refresh_steward_versions_thread(app_handle));

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
