// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::AppConfig;
use prost::Message;
use steward_proto::proto::ScheduleRequest;
use tracing::info;

mod app;
mod schedule;
mod version;

#[tauri::command]
fn version() {
    // TODO: return results to frontend
    version::handle();
}

#[tauri::command]
fn schedule(request_bytes: Vec<u8>) -> Result<(), String> {
    let request = ScheduleRequest::decode(request_bytes.as_slice())
        .map_err(|e| format!("failed to deserialized request: {e:?}"))?;

    schedule::handle(request);

    // TODO: return results to frontend
    Ok(())
}

#[tauri::command]
fn configure(
    somm_node_rpc: &str,
    publisher_domain: &str,
    client_cert_path: &str,
    client_cert_key_path: &str,
) -> String {
    let config = AppConfig {
        grpc_endpoint: Some(somm_node_rpc.to_string()),
        publisher_domain: Some(publisher_domain.to_string()),
        client_cert_path: Some(client_cert_path.to_string()),
        client_cert_key_path: Some(client_cert_key_path.to_string()),
    };

    match app::initialize_app_context(config) {
        Ok(_) => info!("app context initialized"),
        Err(e) => return format!("failed to initialize app config: {e:?}"),
    }

    futures::executor::block_on(async move {
        let app_context = app::get_app_context().await;
        let subscribers = app::get_subscribers(&app_context.grpc_endpoint).await;

        match subscribers {
            Ok(subscribers) => {
                let mut app_context = app::APP_CONTEXT.write().await;
                app_context.subscribers = Some(subscribers);
                info!("subscribers loaded");
            }
            Err(e) => return format!("failed to load subscribers: {e:?}"),
        }

        "app context initialized".to_string()
    })
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![version, schedule, configure])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
