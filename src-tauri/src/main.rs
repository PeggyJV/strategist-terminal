// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::config::AppConfig;
use tauri_plugin_log::LogTarget;

mod adaptors;
mod application;
mod cellar_call;
mod commands;
mod config;
mod lifecycle;
mod logging;
mod schedule;
mod sommelier;
mod state;
mod steward;

fn main() {
    tauri::Builder::default()
        // Initialize state
        .manage(application::Context::new())
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

            // Initialize app context with loaded config
            let config = AppConfig::load();
            if let Err(err) = tauri::async_runtime::block_on(application::apply_config(
                app_handle.clone(),
                config,
            )) {
                log::error!("failed to apply config: {err}");
                std::process::exit(1);
            }

            // Spawn background threads. We run the refresh_subscriber_cache_thread first,
            // then spawn the other two threads.
            tauri::async_runtime::spawn(async move {
                // Populate subscriber cache
                application::refresh_subscriber_cache(app_handle.clone())
                    .await
                    .expect("Failed to refresh subscriber cache");

                // Monitor subscribers' Steward versions
                tauri::async_runtime::spawn(steward::refresh_steward_versions_thread(
                    app_handle.clone(),
                ));

                // Refresh block height
                tauri::async_runtime::spawn(sommelier::refresh_block_height_thread(
                    app_handle.clone(),
                ));

                // Continue running the refresh_subscriber_cache_thread
                tauri::async_runtime::spawn(application::refresh_subscriber_cache_thread(
                    app_handle.clone(),
                ));
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
