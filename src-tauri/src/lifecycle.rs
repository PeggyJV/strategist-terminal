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
    let id = request.id;
    let requests = app_handle.state::<Requests>();

    requests.add(request).await;

    while let Some(status) = status_rx.recv().await {
        let mut requests = requests.0.lock().await;

        // Handle any state updates included in the status
        match &status {
            RequestStatus::AwaitingVote((cork_id, invalidation_scope)) => {
                requests.entry(id).and_modify(|r| {
                    r.cork_id = Some(cork_id.to_owned());
                    r.invalidate_scope = Some(invalidation_scope.to_owned());
                });
            }
            RequestStatus::AwaitingRelay(somm_tx_hash) => {
                requests
                    .entry(id)
                    .and_modify(|r| r.relay_request_tx_hash = somm_tx_hash.to_owned());
            }
            RequestStatus::Relayed(gmp_tx_hash) => {
                requests
                    .entry(id)
                    .and_modify(|r| r.gmp_tx_hash = Some(gmp_tx_hash.to_owned()));
            }
            RequestStatus::FailedExecution(tx_hash) | RequestStatus::Success(tx_hash) => {
                requests
                    .entry(id)
                    .and_modify(|r| r.target_tx_hash = Some(tx_hash.to_owned()));
            }
            _ => (),
        };

        // Update the request state
        requests
            .entry(id)
            .and_modify(|r| r.status = status.to_owned());
    }

    Ok(())
}
