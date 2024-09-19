//! Defines Tauri commands for the frontend to call.

use std::{collections::HashMap, str::FromStr};

use alloy_primitives::Address;
use tauri::Manager;

use crate::{
    application,
    cellar_call::CellarCallData,
    config::AppConfig,
    schedule::{self, build_flash_loan_request, build_request},
    state::{self, RequestState},
};

/// Applies the user's configuration.
#[tauri::command]
pub(crate) async fn configure(
    app_handle: tauri::AppHandle,
    somm_node_rpc: String,
    somm_node_grpc: String,
    publisher_domain: String,
    client_cert_path: String,
    client_cert_key_path: String,
) -> String {
    let config = AppConfig {
        rpc_endpoint: Some(somm_node_rpc),
        grpc_endpoint: Some(somm_node_grpc),
        publisher_domain: Some(publisher_domain),
        client_cert_path: Some(client_cert_path),
        client_cert_key_path: Some(client_cert_key_path),
    };

    // Updates config in state
    application::apply_config(app_handle.clone(), config.clone()).await;

    // Updates config on disk
    if let Err(e) = config.save() {
        return format!("Failed to save config: {}", e);
    }

    "app config applied and saved to file".to_string()
}

/// Gets the current state of all requests.
#[tauri::command]
pub(crate) fn request_state(app_handle: tauri::AppHandle) -> String {
    let state = app_handle.state::<state::Requests>();
    let state_future = state.0.lock();
    let state = futures::executor::block_on(state_future);
    let requests = state
        .iter()
        .map(|(k, v)| (k.to_string(), v.clone()))
        .collect::<HashMap<String, RequestState>>();

    serde_json::to_string(&requests).unwrap()
}

/// Removes a request from state by ID.
#[tauri::command]
pub(crate) fn remove_request(
    app_handle: tauri::AppHandle,
    request_id: String,
) -> Result<(), String> {
    let state = app_handle.state::<state::Requests>();
    let state_future = state.0.lock();
    let mut state = futures::executor::block_on(state_future);
    state.remove(&request_id);

    Ok(())
}

/// Clears all requests from state.
#[tauri::command]
pub(crate) fn clear_requests(app_handle: tauri::AppHandle) -> Result<(), String> {
    let state = app_handle.state::<state::Requests>();
    let state_future = state.0.lock();
    let mut state = futures::executor::block_on(state_future);
    state.clear();

    Ok(())
}

/// Schedules a request to be broadcast to all subscribers and tracked.
#[tauri::command]
pub(crate) async fn schedule_request(
    app_handle: tauri::AppHandle,
    cellar_id: String,
    block_height: String,
    chain_id: String,
    deadline: String,
    flash_loan_call: Option<CellarCallData>,
    queue: Vec<CellarCallData>,
) -> Result<(), String> {
    let queue_log = queue
        .clone()
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    log::trace!(
        cellar_id = cellar_id.as_str(),
        block_height = block_height.as_str(),
        chain_id = chain_id.as_str(),
        deadline = deadline.as_str(),
        queue = queue_log.as_str(),
        flash_loan_call = flash_loan_call.is_some();
        "entered schedule_request handler"
    );

    // parse block_height, chain_id, and deadline as u64
    let block_height = block_height.parse::<u64>().map_err(|e| e.to_string())?;
    let chain_id = chain_id.parse::<u64>().map_err(|e| e.to_string())?;
    let deadline = deadline.parse::<u64>().map_err(|e| e.to_string())?;

    schedule::validate(&cellar_id, block_height, chain_id, deadline, &queue)
        .map_err(|e| e.to_string())?;

    if let Some(mut flash_loan_call) = flash_loan_call {
        if flash_loan_call.adaptor.as_mut().expect("Adaptor address is undefined").is_empty() {
            return Err(String::from("Adaptor address is is empty"));
        }

        let request = build_flash_loan_request(
            cellar_id,
            block_height,
            chain_id,
            deadline,
            flash_loan_call,
            queue,
        )
        .map_err(|e| e.to_string())?;

        println!("request: {:?}", request);

        // schedule::handle(app_handle.clone(), request);

        return Ok(());
    }

    log::trace!("building request");

    let request = match build_request(cellar_id, block_height, chain_id, deadline, queue) {
        Ok(r) => r,
        Err(e) => {
            log::error!("error building request: {:?}", e);
            return Err(e.to_string());
        }
    };

    log::trace!(request:?; "spawning request handler");

    // schedule::handle(app_handle.clone(), request)
    //     .await
    //     .map_err(|e| e.to_string())
    return Ok(());
}

/// Command to get all subscribers' current Steward versions.
#[tauri::command]
pub(crate) fn steward_versions(app_handle: tauri::AppHandle) -> HashMap<String, String> {
    let state = app_handle.state::<state::Stewards>();
    let state_future = state.0.lock();
    let state = futures::executor::block_on(state_future);

    state.clone().versions
}

#[tauri::command]
pub(crate) fn get_app_config() -> Result<AppConfig, String> {
    let config = AppConfig::load();
    Ok(config)
}
