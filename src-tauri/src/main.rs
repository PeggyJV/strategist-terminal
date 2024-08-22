// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{collections::HashMap, fmt::Pointer, str::FromStr};

use alloy_primitives::Address;
use app::AppConfig;

use cellar_call::CellarCall;
use log::{kv::Visitor, trace};
use schedule::{build_request, validate_calls};

use crate::schedule::build_flash_loan_request;
use tauri::Manager;
use tauri_plugin_log::LogTarget;
use tracing::info;

use crate::state::RequestState;

mod adaptors;
mod app;
mod cellar_call;
mod lifecycle;
mod schedule;
mod sommelier;
mod state;
mod steward;
mod version;

#[tauri::command]
fn version() {
    // TODO: return results to frontend
    version::handle();
}

#[tauri::command]
fn schedule_request(
    app_handle: tauri::AppHandle,
    cellar_id: String,
    block_height: String,
    chain_id: String,
    deadline: String,
    flash_loan_call: Option<CellarCall>,
    queue: Vec<CellarCall>,
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

    log::trace!("validating calls");

    if let Err(err) = validate_calls(&queue) {
        log::error!("error validating calls: {:?}", err);
        return Err(err.to_string());
    }

    if let Some(flash_loan_call) = flash_loan_call {
        if flash_loan_call.adaptor.is_empty() {
            return Err(String::from("adaptor id is empty"));
        }

        if Address::from_str(&flash_loan_call.adaptor).is_err() {
            return Err(String::from("invalid adaptor address"));
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

        //schedule::handle(request);

        return Ok(());
    }
    println!("Flashloan false!");

    log::trace!("building request");

    let request = match build_request(cellar_id, block_height, chain_id, deadline, queue) {
        Ok(r) => r,
        Err(e) => {
            log::error!("error building request: {:?}", e);
            return Err(e.to_string());
        }
    };

    log::trace!(request:?; "spawning request handler");

    tokio::task::spawn(schedule::handle(request, app_handle));

    // TODO: return results to frontend
    Ok(())
}

#[tauri::command]
fn configure(
    app_handle: tauri::AppHandle,
    somm_node_rpc: &str,
    somm_node_grpc: &str,
    publisher_domain: &str,
    client_cert_path: &str,
    client_cert_key_path: &str,
) -> String {
    // Run the block sync thread. Doing this here because it requires a gRPC endpoint, would be
    // nice if it worked at startup though.
    tokio::task::spawn(sommelier::sync_block_height(
        app_handle,
        somm_node_rpc.to_string(),
    ));

    let config = AppConfig {
        rpc_endpoint: Some(somm_node_rpc.to_string()),
        grpc_endpoint: Some(somm_node_grpc.to_string()),
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
        .manage(state::Sommelier::new())
        .manage(state::Requests::new())
        .plugin(
            tauri_plugin_log::Builder::default()
                .format(|out, message, record| {
                    let mut key_values = KVVisitor::default();

                    record.key_values().visit(&mut key_values).unwrap();

                    out.finish(format_args!(
                        "[{}] [{}] [{}] {} {}",
                        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                        record.target(),
                        record.level(),
                        message,
                        key_values,
                    ))
                })
                .targets([LogTarget::Stdout])
                .level(log::LevelFilter::Info)
                .level_for("app", log::LevelFilter::Trace)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            version,
            schedule_request,
            configure
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Debug, Default)]
struct KVVisitor {
    pub map: HashMap<String, String>,
}

impl std::fmt::Display for KVVisitor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.map)
    }
}

impl<'kvs> log::kv::VisitSource<'kvs> for KVVisitor {
    fn visit_pair(
        &mut self,
        key: log::kv::Key<'kvs>,
        value: log::kv::Value<'kvs>,
    ) -> Result<(), log::kv::Error> {
        self.map.insert(key.to_string(), value.to_string());
        Ok(())
    }
}
