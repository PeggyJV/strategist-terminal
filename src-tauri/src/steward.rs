use eyre::Result;
use tauri::Manager;

use crate::{
    app::{self, AppContext},
    state,
};

pub(crate) struct StewardVersion {
    pub(crate) endpoint: String,
    pub(crate) version: Option<String>,
}

/// Queries all subscriber endpoints for their Steward verions
pub(crate) async fn get_all_steward_versions(
    app_context: &AppContext,
) -> Vec<StewardVersion> {
    let Some(subscribers) = &app_context.subscribers else {
        log::error!("no subscribers found when attempting to query steward versions");
        return vec![];
    };

    futures::future::join_all(
        subscribers.iter().map(|s| get_steward_version(s.push_url.clone()))
    ).await
}

/// Queries the provided subscriber endpoint for it's Steward version
pub(crate) async fn get_steward_version(grpc_endpoint: String) -> StewardVersion {
    let mut client = match steward_proto::proto::status_service_client::StatusServiceClient::connect(
        grpc_endpoint.clone(),
    )
    .await {
        Ok(client) => client,
        Err(e) => {
            log::error!(message = e.to_string(), push_url = grpc_endpoint.as_str(); "failed to connect to steward");

            return StewardVersion {
                endpoint: grpc_endpoint,
                version: None,
            };
        }
    };
    let request = tonic::Request::new(steward_proto::proto::VersionRequest {});

    log::debug!(push_url = grpc_endpoint.as_str(); "getting steward version");

    match client.version(request).await {
        Ok(response) => {
            let response = response.into_inner();
            let version = response.version;

            log::trace!(version = version.as_str(), push_url = grpc_endpoint.as_str(); "steward version retrieved");

            StewardVersion {
                endpoint: grpc_endpoint,
                version: Some(version),
            }
        }
        Err(e) => {
            log::error!(code = e.code() as i32, message = e.message(), push_url = grpc_endpoint.as_str(); "failed to get steward version");

            StewardVersion {
                endpoint: grpc_endpoint,
                version: None,
            }
        }
    }
}

/// Updates state with current Steward versions
pub(crate) async fn refresh_steward_versions(app_handle: tauri::AppHandle) {
    log::trace!("refreshing steward versions");

    let app_context = app::get_app_context().await;
    let versions = get_all_steward_versions(&app_context).await;
    let state = app_handle.state::<state::Stewards>();
    let mut state = state.0.lock().await;
    state.versions = versions
        .into_iter()
        .map(|v| (v.endpoint, v.version.unwrap_or_else(|| "unavailable".to_string())))
        .collect();

    log::debug!("refreshed steward versions: {:?}", state.versions);
}

/// Thread to refresh steward versions in the background
pub(crate) async fn refresh_steward_versions_thread(app_handle: tauri::AppHandle) {
    loop {
        refresh_steward_versions(app_handle.clone()).await;

        tokio::time::sleep(std::time::Duration::from_secs(600)).await;
    }
}
