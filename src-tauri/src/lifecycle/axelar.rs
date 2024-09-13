use eyre::{bail, Result};
use steward_proto::proto::ScheduleRequest;
use tauri::async_runtime::Sender;

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

    relay::monitor_ibc_relay(app_handle, trace_id, tx.clone(), chain_id, block_height).await?;

    Ok(())
}
