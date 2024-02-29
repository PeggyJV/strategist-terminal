// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use prost::Message;
use steward_proto::proto::ScheduleRequest;

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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![version])
        .invoke_handler(tauri::generate_handler![schedule])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
