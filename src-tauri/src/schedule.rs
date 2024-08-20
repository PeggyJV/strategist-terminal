use std::str::FromStr;

use alloy_primitives::Address;
use eyre::{bail, Result};
use futures::StreamExt;
use somm_proto::pubsub::Subscriber;
use steward_proto::proto::{
    contract_call_service_client::ContractCallServiceClient, ScheduleRequest, ScheduleResponse,
};
use tauri::{async_runtime::Sender, Manager};
use tonic::transport::{Channel, Identity};
use tracing::{debug, error, info};

use crate::{
    app::{self, get_channel, AppContext},
    cellar_call::{construct_call_data, CellarCall},
    lifecycle,
    state::{RequestState, RequestStatus, Requests},
};

pub(crate) fn validate_calls(calls: &Vec<CellarCall>) -> Result<()> {
    if calls.is_empty() {
        bail!("cellar call data is empty");
    };

    for call in calls.iter() {
        if call.adaptor.is_empty() {
            bail!("adaptor id is empty");
        }

        if Address::from_str(&call.adaptor).is_err() {
            bail!("invalid adaptor address");
        }
    }

    Ok(())
}

pub(crate) fn build_request(
    cellar_id: String,
    block_height: u64,
    chain_id: u64,
    deadline: u64,
    queue: Vec<CellarCall>,
) -> Result<ScheduleRequest> {
    let adaptor_calls = queue
        .into_iter()
        .map(|call| call.to_adaptor_call())
        .collect::<Result<_>>()?;
    let call_data = Some(construct_call_data(adaptor_calls));

    Ok(ScheduleRequest {
        cellar_id,
        chain_id,
        block_height,
        deadline,
        call_data,
    })
}

/// Handles submitting and tracking the request
pub(crate) async fn handle(request: ScheduleRequest, app_handle: tauri::AppHandle) -> Result<()> {
    let app_context = app::get_app_context().await;
    let request_state = RequestState::new();
    let (tx, rx) = tokio::sync::mpsc::channel::<RequestStatus>(1);
    let chain_id = request.chain_id;

    let trace_id = request_state.id.to_string();
    let trace_id = trace_id.as_str();
    log::trace!(id = trace_id, chain_id; "handling request");

    // Run lifecycle tracking thread
    tokio::spawn(lifecycle::track_request(
        app_handle.clone(),
        request_state,
        rx,
    ));

    // Broadcast the request to all subscribers
    tx.send(RequestStatus::Broadcasting).await?;

    let height = request.block_height;
    let request_status =
        broadcast_schedule_request(trace_id, &app_context, tx.clone(), request).await?;
    let (cork_id, invalidation_scope) = match request_status {
        RequestStatus::FailedBroadcast => {
            log::warn!(id = trace_id; "broadcast failed");

            return Ok(());
        }
        RequestStatus::AwaitingVote((cork_id, invalidation_scope)) => {
            (cork_id.clone(), invalidation_scope.clone())
        }
        _ => bail!("unexpected request status after broadcast: {request_status:?}"),
    };

    // Wait for the scheduled height
    // let request_status =
    //     cork_voting_period(app_handle, &app_context, &cork_id, height, tx.clone()).await?;
    // let tx = match request_status {
    //     RequestStatus::AwaitingRelay(tx) => tx,
    //     RequestStatus::FailedVote => return Ok(()),
    //     _ => bail!("unexpected request status after voting period: {request_status:?}"),
    // };

    // // Wait for relay
    // if invalidation_scope.is_empty() {
    //     log::error!("invalidation scope is empty");
    //     return Ok(());
    // }

    Ok(())
}

/// Broadcasts the [`ScheduleRequest`] to all subscribers
async fn broadcast_schedule_request(
    id: &str,
    context: &AppContext,
    tx: Sender<RequestStatus>,
    request: ScheduleRequest,
) -> Result<RequestStatus> {
    log::trace!(id; "broadcasting schedule request");

    let Some(subscribers) = context.subscribers.to_owned() else {
        log::error!(id; "no subscribers found");

        tx.send(RequestStatus::FailedBroadcast).await?;
        return Ok(RequestStatus::FailedBroadcast);
    };

    let Some(identity) = context.client_identity.clone() else {
        log::error!(id; "client identity not found");
        tx.send(RequestStatus::FailedBroadcast).await?;
        bail!("app context does not contain client identity. user must provide their certificate and key.");
    };

    let posts = futures::stream::iter(subscribers.into_iter().map(|subscriber| {
        tokio::spawn(handle_subscriber(
            identity.clone(),
            subscriber,
            request.clone(),
        ))
    }))
    .buffer_unordered(10)
    .collect::<Vec<_>>();

    let result = posts.await;

    // Extract the cork ID from one of the successful responses.
    // If there were no successful responses this iterator has length zero and nothing happens,
    // resulting in a failed broadcast.
    for (cid, is) in result.into_iter().flatten().flatten() {
        tx.send(RequestStatus::AwaitingVote((cid.clone(), is.clone())))
            .await?;
        return Ok(RequestStatus::AwaitingVote((cid.clone(), is.clone())));
    }

    log::trace!(id; "broadcasting schedule request failed");
    tx.send(RequestStatus::FailedBroadcast).await?;

    Ok(RequestStatus::FailedBroadcast)
}

/// Returns the (Cork ID, Invalidation Scope) tuple from the subscriber
async fn handle_subscriber(
    identity: Identity,
    subscriber: Subscriber,
    request: ScheduleRequest,
) -> Result<(String, String)> {
    debug!(
        "sending contract call to subscriber {}",
        &subscriber.push_url
    );

    let request = request.clone();
    let channel = get_channel(identity, subscriber.ca_cert, subscriber.push_url.clone()).await?;

    info!("sending schedule call to {}", subscriber.push_url);
    let response = match schedule(channel, request).await {
        Ok(res) => {
            debug!("response from {}: {res:?}", subscriber.push_url);
            res
        }
        Err(err) => bail!(
            "failed to send contract call to subscriber {}: {err:?}",
            subscriber.address
        ),
    };

    let response = response.into_inner();

    Ok((response.id, response.invalidation_scope))
}

async fn schedule(
    channel: Channel,
    request: ScheduleRequest,
) -> Result<tonic::Response<ScheduleResponse>> {
    let mut client = ContractCallServiceClient::new(channel);

    client.schedule(request).await.map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tokio::task::JoinSet;

    use super::*;

    #[tokio::test]
    async fn test_get_channel() {
        let client_cert_path = "tests/tls/test_client.crt".to_string();
        let client_cert = fs::read_to_string(client_cert_path).unwrap();
        let client_key_path = "tests/tls/test_client_key_pkcs8.pem".to_string();
        let client_key = fs::read_to_string(client_key_path).unwrap();
        let identity = Identity::from_pem(client_cert, client_key);
        let subscriber_ca = "-----BEGIN CERTIFICATE-----
MIICWDCCAd6gAwIBAgIUcVWY42nJqQFpKpNsgaRB/eR1QhcwCgYIKoZIzj0EAwMw
YzELMAkGA1UEBhMCQVUxEzARBgNVBAgMClNvbWUtU3RhdGUxITAfBgNVBAoMGElu
dGVybmV0IFdpZGdpdHMgUHR5IEx0ZDEcMBoGA1UEAwwTY2hhcmRvbm5heTAuc29t
bS5maTAeFw0yNDAyMjgxNTM5MTRaFw0yNjAyMjcxNTM5MTRaMGMxCzAJBgNVBAYT
AkFVMRMwEQYDVQQIDApTb21lLVN0YXRlMSEwHwYDVQQKDBhJbnRlcm5ldCBXaWRn
aXRzIFB0eSBMdGQxHDAaBgNVBAMME2NoYXJkb25uYXkwLnNvbW0uZmkwdjAQBgcq
hkjOPQIBBgUrgQQAIgNiAAQRFDVhnKIXSXKgJR7rlIcj1Eb0eEzsV2PZXLdPaX9L
uRYWEg7k3Jz20QZZa3hFQkRFBwrpaYGezLgM4/eVB2N4+1WFBoTFZFR2cZ0RXpBe
8CMwqDVnJGKjeNJ6GgQpFgmjUzBRMB0GA1UdDgQWBBS/x133VMIL4Hdpcp9KP2LI
HVbgNTAfBgNVHSMEGDAWgBS/x133VMIL4Hdpcp9KP2LIHVbgNTAPBgNVHRMBAf8E
BTADAQH/MAoGCCqGSM49BAMDA2gAMGUCMQCHr4CQw8oIitDqm/wQthH9CshezvlF
oBA+9gAfZIZxqDzomWPySAp+Z3vsKQ5o9sUCMBcwPd4vFQcMgCKr0Eg5g2ON1g55
5LOIFNUwp2dIALMYCvuQM+rPvQMfRPeVADKsJw==
-----END CERTIFICATE-----"
            .to_string();
        let subscriber_push_url = "chardonnay0.somm.fi:5734".to_string();

        get_channel(identity, subscriber_ca, subscriber_push_url)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_handle_scheduling() {
        let client_cert_path = "tests/tls/test_client.crt".to_string();
        let client_cert = fs::read_to_string(client_cert_path).unwrap();
        let client_key_path = "tests/tls/test_client_key_pkcs8.pem".to_string();
        let client_key = fs::read_to_string(client_key_path).unwrap();
        let identity = Identity::from_pem(client_cert, client_key);
        let subscriber = Subscriber {
            address: "somm199sjfhaw3hempwzljw0lgwsm9kk6r8e5vhl2uw".to_string(),
            push_url: "chardonnay0.somm.fi:5734".to_string(),
            ca_cert: "-----BEGIN CERTIFICATE-----
MIICWDCCAd6gAwIBAgIUcVWY42nJqQFpKpNsgaRB/eR1QhcwCgYIKoZIzj0EAwMw
YzELMAkGA1UEBhMCQVUxEzARBgNVBAgMClNvbWUtU3RhdGUxITAfBgNVBAoMGElu
dGVybmV0IFdpZGdpdHMgUHR5IEx0ZDEcMBoGA1UEAwwTY2hhcmRvbm5heTAuc29t
bS5maTAeFw0yNDAyMjgxNTM5MTRaFw0yNjAyMjcxNTM5MTRaMGMxCzAJBgNVBAYT
AkFVMRMwEQYDVQQIDApTb21lLVN0YXRlMSEwHwYDVQQKDBhJbnRlcm5ldCBXaWRn
aXRzIFB0eSBMdGQxHDAaBgNVBAMME2NoYXJkb25uYXkwLnNvbW0uZmkwdjAQBgcq
hkjOPQIBBgUrgQQAIgNiAAQRFDVhnKIXSXKgJR7rlIcj1Eb0eEzsV2PZXLdPaX9L
uRYWEg7k3Jz20QZZa3hFQkRFBwrpaYGezLgM4/eVB2N4+1WFBoTFZFR2cZ0RXpBe
8CMwqDVnJGKjeNJ6GgQpFgmjUzBRMB0GA1UdDgQWBBS/x133VMIL4Hdpcp9KP2LI
HVbgNTAfBgNVHSMEGDAWgBS/x133VMIL4Hdpcp9KP2LIHVbgNTAPBgNVHRMBAf8E
BTADAQH/MAoGCCqGSM49BAMDA2gAMGUCMQCHr4CQw8oIitDqm/wQthH9CshezvlF
oBA+9gAfZIZxqDzomWPySAp+Z3vsKQ5o9sUCMBcwPd4vFQcMgCKr0Eg5g2ON1g55
5LOIFNUwp2dIALMYCvuQM+rPvQMfRPeVADKsJw==
-----END CERTIFICATE-----"
                .to_string(),
        };
        let request = ScheduleRequest {
            cellar_id: "0xf9d0bb4fE3a004bE6766005EE9Fb889A8A0DCED3".to_string(),
            chain_id: 42161,
            block_height: 12345689,
            deadline: 12345689101112,
            call_data: None,
        };
        let mut set = JoinSet::new();

        set.spawn(handle_subscriber(
            identity.clone(),
            subscriber.clone(),
            request.clone(),
        ));
        set.spawn(handle_subscriber(
            identity.clone(),
            subscriber.clone(),
            request.clone(),
        ));
        set.spawn(handle_subscriber(
            identity.clone(),
            subscriber.clone(),
            request.clone(),
        ));

        while let Some(res) = set.join_next().await {
            info!("result: {res:?}");
        }
    }
}
