// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::str::FromStr;

use alloy_primitives::Address;
use app::AppConfig;

use cellar_call::CellarCall;
use schedule::{build_request, validate_calls, ScheduleRequestData};

use tracing::info;

mod adaptors;
mod app;
mod cellar_call;
mod schedule;
mod version;

#[tauri::command]
fn version() {
    // TODO: return results to frontend
    version::handle();
}

#[tauri::command]
fn schedule_request(
    cellar_id: String,
    block_height: String,
    chain_id: String,
    deadline: String,
    queue: Vec<CellarCall>,
) -> Result<(), String> {
    // print all of the arguments
    println!("cellar_id: {}", cellar_id);
    println!("block_height: {}", block_height);
    println!("chain_id: {}", chain_id);
    println!("deadline: {}", deadline);
    println!("queue: {:?}", queue);

    // parse block_height, chain_id, and deadline as u64
    let block_height = block_height.parse::<u64>().map_err(|e| e.to_string())?;
    let chain_id = chain_id.parse::<u64>().map_err(|e| e.to_string())?;
    let deadline = deadline.parse::<u64>().map_err(|e| e.to_string())?;

    if Address::from_str(&cellar_id).is_err() {
        return Err(String::from("invalid cellar address"));
    }

    if block_height == 0 {
        return Err(String::from("block height cannot be zero"));
    }

    if chain_id == 0 {
        return Err(String::from("invalid chain id"));
    }

    if deadline == 0 {
        return Err(String::from("deadline cannot be zero"));
    }

    validate_calls(&queue).map_err(|e| e.to_string())?;

    let request = build_request(cellar_id, block_height, chain_id, deadline, queue)
        .map_err(|e| e.to_string())?;

    //schedule::handle(request);

    println!("request: {:?}", request);

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
        .invoke_handler(tauri::generate_handler![
            version,
            schedule_request,
            configure
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
