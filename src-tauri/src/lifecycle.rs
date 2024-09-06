//! For tracking the lifecycle of different requests and updating their state for the frontend

use eyre::Result;
use tauri::Manager;
use tokio::sync::mpsc::Receiver;

use crate::state::{RequestState, RequestStatus, Requests};

pub(crate) mod cork_vote;
pub(crate) mod gravity;
pub(crate) mod relay;

/// Tracks and updates a request's state
pub async fn track_request(
    app_handle: tauri::AppHandle,
    request: RequestState,
    mut status_rx: Receiver<RequestStatus>,
) -> Result<()> {
    let id = request.id.clone();
    let trace_id = id.as_str();
    let requests = app_handle.state::<Requests>();

    log::info!(id = trace_id, status = &request.status; "tracking request");

    requests.add(request).await;

    while let Some(status) = status_rx.recv().await {
        let mut requests = requests.0.lock().await;

        // Handle any state updates included in the status
        match &status {
            RequestStatus::AwaitingVote((cork_id, invalidation_scope)) => {
                requests.entry(id.clone()).and_modify(|r| {
                    r.cork_id = Some(cork_id.to_owned());
                    r.invalidate_scope = Some(invalidation_scope.to_owned());
                });

                log::info!(id = trace_id, status = &RequestStatus::AwaitingVote((cork_id.to_owned(), invalidation_scope.to_owned())); "request is awaiting vote");
            }
            RequestStatus::AwaitingConfirmation => {
                requests
                    .entry(id.clone())
                    .and_modify(|r| r.status = RequestStatus::AwaitingConfirmation);

                log::info!(id = trace_id, status = &RequestStatus::AwaitingConfirmation; "request is awaiting confirmation");
            }
            RequestStatus::FailedVote => {
                requests
                    .entry(id.clone())
                    .and_modify(|r| r.status = RequestStatus::FailedVote);

                log::error!(id = trace_id, status = &RequestStatus::FailedVote; "request failed vote");
            }
            RequestStatus::AwaitingRelay(somm_tx_hash) => {
                requests
                    .entry(id.clone())
                    .and_modify(|r| r.relay_request_tx_hash = somm_tx_hash.to_owned());

                log::info!(id = trace_id, status = &RequestStatus::AwaitingRelay(somm_tx_hash.to_owned()); "request is awaiting relay");
            }
            RequestStatus::Relayed(gmp_tx_hash) => {
                requests
                    .entry(id.clone())
                    .and_modify(|r| r.gmp_tx_hash = Some(gmp_tx_hash.to_owned()));

                log::info!(id = trace_id, status = &RequestStatus::Relayed(gmp_tx_hash.to_owned()); "request is relayed");
            }
            RequestStatus::FailedExecution(tx_hash) | RequestStatus::Success(tx_hash) => {
                requests
                    .entry(id.clone())
                    .and_modify(|r| r.target_tx_hash = Some(tx_hash.to_owned()));

                log::info!(id = trace_id, status = &RequestStatus::FailedExecution(tx_hash.to_owned()); "request failed execution");
            }
            _ => {
                log::info!(id = trace_id, status = &status; "request has status unhandled by tracker");
            }
        };

        // Update the request state
        requests
            .entry(id.clone())
            .and_modify(|r| r.status = status.to_owned());
    }

    log::info!(id = trace_id; "request tracking ended");

    Ok(())
}
