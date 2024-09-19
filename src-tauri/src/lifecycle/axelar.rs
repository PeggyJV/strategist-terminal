use std::time::Duration;

use axelar_gmp::{
    builder::SearchGMPRequestBuilder,
    search::{self, SearchGMPRequest},
};
use eyre::{bail, Result};
use steward_proto::proto::ScheduleRequest;
use tauri::async_runtime::Sender;
use tokio::time::Interval;

use crate::state::RequestStatus;

use super::{
    cork_vote::{self, CorkType},
    relay,
};

pub(crate) async fn track_axelar_cork(
    app_handle: tauri::AppHandle,
    trace_id: String,
    tx: Sender<RequestStatus>,
    request: ScheduleRequest,
    request_status: RequestStatus,
) -> Result<()> {
    let cork_id = match request_status {
        RequestStatus::AwaitingVote((cork_id, _)) => cork_id,
        _ => bail!("{trace_id} has unexpected request status after broadcast: {request_status:?}"),
    };

    // Watch the contract for call execution
    // Perhaps use Axelar's API for tracking the lifecycle?

    // Wait for the scheduled height
    let chain_id = request.chain_id;
    let block_height = request.block_height;

    cork_vote::monitor_cork_voting_period(
        app_handle.clone(),
        trace_id.clone(),
        CorkType::Axelar(cork_id.clone(), chain_id),
        block_height,
        tx.clone(),
    )
    .await?;

    let RequestStatus::AwaitingRelay(tx_hash) = relay::monitor_ibc_relay(
        app_handle.clone(),
        trace_id.clone(),
        tx.clone(),
        chain_id,
        request.cellar_id.clone(),
        block_height,
    )
    .await?
    else {
        log::warn!(id = trace_id, chain_id, cellar_id = request.cellar_id; "lost track of request status");
        return Ok(());
    };

    monitor_axelar_gmp(trace_id.clone(), tx.clone(), tx_hash).await?;

    Ok(())
}

pub(crate) async fn monitor_axelar_gmp(
    trace_id: String,
    tx: Sender<RequestStatus>,
    tx_hash: String,
) -> Result<RequestStatus> {
    let timeout = tokio::time::sleep(Duration::from_secs(3600)); // 1 hour

    tokio::select! {
        result = monitor_status(trace_id.clone(), tx.clone(), tx_hash.clone()) => result,
        _ = timeout => return timeout_result(trace_id, tx, tx_hash).await
    }
}

async fn timeout_result(
    trace_id: String,
    tx: Sender<RequestStatus>,
    tx_hash: String,
) -> Result<RequestStatus> {
    log::warn!(id = trace_id, tx_hash = tx_hash; "Axelar GMP transaction monitoring timed out after 1 hour");
    tx.send(RequestStatus::Unknown).await?;
    Ok(RequestStatus::Unknown)
}

async fn monitor_status(
    trace_id: String,
    tx: Sender<RequestStatus>,
    tx_hash: String,
) -> Result<RequestStatus> {
    let mut status: RequestStatus;

    loop {
        status = get_status(trace_id.clone(), tx.clone(), tx_hash.clone()).await?;
        tx.send(status.clone()).await?;

        match status {
            RequestStatus::Success(_)
            | RequestStatus::Unknown
            | RequestStatus::FailedExecution(_) => break,
            _ => tokio::time::sleep(Duration::from_secs(10)).await,
        }
    }

    Ok(status)
}

async fn get_status(
    trace_id: String,
    tx: Sender<RequestStatus>,
    tx_hash: String,
) -> Result<RequestStatus> {
    let mut search_result = SearchGMPRequestBuilder::new()
        .tx_hash(tx_hash.clone())
        .build()
        .send()
        .await?;

    while (search_result.data.is_none() || search_result.data.clone().unwrap().is_empty()) {
        tokio::time::sleep(Duration::from_secs(10)).await;
        search_result = SearchGMPRequestBuilder::new()
            .tx_hash(tx_hash.clone())
            .build()
            .send()
            .await?;
    }

    let gmp_transaction = search_result.data.unwrap()[0].clone();
    let status = gmp_transaction.status.clone();

    match status.as_str() {
        "confirming"
        | "approving"
        | "approved"
        | "executing"
        | "waiting_for_route_message"
        | "waiting_for_ibc"
        | "insufficient_fee" => {
            log::info!(id = trace_id, tx_hash = tx_hash, status = status; "Axelar GMP transaction in progress");
            return Ok(RequestStatus::AwaitingExecution((
                tx_hash.clone(),
                status.clone(),
            )));
        }
        "express_executed" | "executed" => {
            log::info!(id = trace_id, tx_hash = tx_hash, status = status; "Axelar GMP transaction executed successfully");
            let target_tx_hash = gmp_transaction.call.transaction_hash.clone();
            return Ok(RequestStatus::Success(target_tx_hash));
        }
        "error" => {
            log::error!(id = trace_id, tx_hash = tx_hash, status = status; "Axelar GMP transaction failed");
            let target_tx_hash = gmp_transaction.call.transaction_hash.clone();
            return Ok(RequestStatus::FailedExecution(target_tx_hash));
        }
        _ => {
            log::warn!(id = trace_id, tx_hash = tx_hash, status = status; "Unknown Axelar GMP transaction status");
            return Ok(RequestStatus::Unknown);
        }
    }
}
