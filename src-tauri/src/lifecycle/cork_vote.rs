use eyre::{bail, Context, Result};
use somm_proto::cork::{
    query_client::QueryClient as CorkQueryClient, QueryCorkResultRequest, QueryCorkResultResponse,
};
use tauri::{async_runtime::Sender, Manager};
use tonic::Response;

use crate::{
    application::AppContext,
    state::{self, RequestStatus},
};

/// Monitor the x/cork module for the result of the cork vote
//pub async fn cork_voting_period(
//    app_handle: tauri::AppHandle,
//    app_context: &AppContext,
//    cork_id: &str,
//    target_height: u64,
//    tx: Sender<RequestStatus>,
//) -> Result<RequestStatus> {
//    let mut current_height = get_current_height(app_handle.clone()).await;
//
//    if current_height == 0 {
//        bail!("block height is not set");
//    }
//
//    while current_height < target_height {
//        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
//        current_height = get_current_height(app_handle.clone()).await;
//    }
//
//    let mut result = get_cork_result(app_context, cork_id).await;
//
//    while result.is_err() {
//        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
//        result = get_cork_result(app_context, cork_id).await;
//    }
//
//    let Some(cork_result) = result.unwrap().into_inner().cork_result else {
//        bail!("cork result was empty");
//    };
//
//    if cork_result.approved {
//        tx.send(RequestStatus::AwaitingRelay(None)).await?;
//
//        return Ok(RequestStatus::AwaitingRelay(None));
//    }
//
//    tx.send(RequestStatus::FailedVote).await?;
//
//    return Ok(RequestStatus::FailedVote);
//}

async fn get_current_height(app_handle: tauri::AppHandle) -> u64 {
    app_handle
        .state::<state::Sommelier>()
        .0
        .lock()
        .await
        .block_height
}

async fn get_cork_result(
    app_context: &AppContext,
    cork_id: &str,
) -> Result<Response<QueryCorkResultResponse>> {
    let mut client = CorkQueryClient::connect(app_context.grpc_endpoint.clone()).await?;

    client
        .query_cork_result(QueryCorkResultRequest {
            id: cork_id.to_string(),
        })
        .await
        .wrap_err("error querying cork result")
}
