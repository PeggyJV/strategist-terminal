use eyre::{bail, Context as _, Result};
use serde::{Deserialize, Serialize};
use somm_proto::axelar_cork::{self, query_client::QueryClient as AxelarCorkQueryClient};
use somm_proto::cork::{
    query_client::QueryClient as CorkQueryClient, QueryCorkResultRequest, QueryCorkResultResponse,
};
use tauri::{async_runtime::Sender, Manager};
use tonic::Response;

use crate::{
    application::Context,
    state::{self, RequestStatus},
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) enum CorkType {
    Gravity(String),
    Axelar(String, u64),
}

impl log::kv::ToValue for CorkType {
    fn to_value(&self) -> log::kv::Value {
        log::kv::Value::from_serde(self)
    }
}

impl std::fmt::Display for CorkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Monitor the x/cork module for the result of the cork vote
pub(crate) async fn monitor_cork_voting_period(
    app_handle: tauri::AppHandle,
    trace_id: String,
    cork_type: CorkType,
    target_height: u64,
    tx: Sender<RequestStatus>,
) -> Result<RequestStatus> {
    log::debug!(id = trace_id, cork_type = &cork_type; "monitoring cork voting period");
    let mut current_height = get_current_height(app_handle.clone()).await;

    if current_height == 0 {
        bail!("block height is not set");
    }

    while current_height < target_height {
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        current_height = get_current_height(app_handle.clone()).await;
    }

    log::debug!(id = trace_id, cork_type = &cork_type, current_height; "cork voting period ended");

    let request_status = match cork_type {
        CorkType::Gravity(cork_id) => {
            handle_gravity_cork_vote_result(app_handle.clone(), trace_id, cork_id).await?
        }
        CorkType::Axelar(cork_id, chain_id) => {
            handle_axelar_cork_vote_result(app_handle.clone(), trace_id, chain_id, cork_id).await?
        }
    };

    tx.send(request_status.clone()).await?;

    return Ok(request_status);
}

async fn handle_gravity_cork_vote_result(
    app_handle: tauri::AppHandle,
    trace_id: String,
    cork_id: String,
) -> Result<RequestStatus> {
    log::debug!(id = trace_id, cork_id; "getting gravity cork result");
    let mut result = get_cork_result(&app_handle, &cork_id).await;
    while result.is_err() {
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        result = get_cork_result(&app_handle, &cork_id).await;
    }

    let Some(cork_result) = result.unwrap().into_inner().cork_result else {
        bail!("cork result was empty");
    };

    if cork_result.approved {
        return Ok(RequestStatus::AwaitingConfirmation);
    }

    return Ok(RequestStatus::FailedVote);
}

async fn handle_axelar_cork_vote_result(
    app_handle: tauri::AppHandle,
    trace_id: String,
    chain_id: u64,
    cork_id: String,
) -> Result<RequestStatus> {
    log::debug!(id = trace_id, chain_id, cork_id; "getting axelar cork result");
    let mut result = get_axelar_cork_result(&app_handle, chain_id, &cork_id).await;
    while result.is_err() {
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        result = get_axelar_cork_result(&app_handle, chain_id, &cork_id).await;
    }

    let Some(cork_result) = result.unwrap().into_inner().cork_result else {
        bail!("cork result was empty");
    };

    if cork_result.approved {
        return Ok(RequestStatus::AwaitingRelayCorkCall);
    }

    return Ok(RequestStatus::FailedVote);
}

async fn get_current_height(app_handle: tauri::AppHandle) -> u64 {
    app_handle
        .state::<state::Sommelier>()
        .0
        .lock()
        .await
        .block_height
}

async fn get_cork_result(
    app_handle: &tauri::AppHandle,
    cork_id: &str,
) -> Result<Response<QueryCorkResultResponse>> {
    let grpc_endpoint = app_handle
        .state::<Context>()
        .0
        .read()
        .await
        .grpc_endpoint
        .clone();

    let mut client = CorkQueryClient::connect(grpc_endpoint).await?;

    client
        .query_cork_result(QueryCorkResultRequest {
            id: cork_id.to_string(),
        })
        .await
        .wrap_err("error querying cork result")
}

async fn get_axelar_cork_result(
    app_handle: &tauri::AppHandle,
    chain_id: u64,
    cork_id: &str,
) -> Result<Response<axelar_cork::QueryCorkResultResponse>> {
    let grpc_endpoint = app_handle
        .state::<Context>()
        .0
        .read()
        .await
        .grpc_endpoint
        .clone();

    let mut client = AxelarCorkQueryClient::connect(grpc_endpoint).await?;

    client
        .query_cork_result(axelar_cork::QueryCorkResultRequest {
            chain_id,
            id: cork_id.to_string(),
        })
        .await
        .wrap_err("error querying cork result")
}
