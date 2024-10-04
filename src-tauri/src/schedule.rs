use std::str::FromStr;

use alloy_primitives::Address;
use eyre::{bail, Result};
use futures::StreamExt;
use serde::Deserialize;
use serde_json::Value;
use somm_proto::pubsub::Subscriber;
use steward_proto::proto::{
    aave_v3_debt_token_adaptor_v1_flash_loan::AdaptorCallForAaveV3FlashLoan,
    balancer_pool_adaptor_v1_flash_loan::AdaptorCallForBalancerPoolFlashLoan,
    contract_call_service_client::ContractCallServiceClient,
    AdaptorCall,
    ScheduleRequest,
    ScheduleResponse,
    cellar_v2_5
};
use tauri::async_runtime::Sender;
use tauri::Manager;
use tonic::transport::{Channel, Identity};
use tracing::{debug, info};

use crate::adaptors::{
    get_aave_v3_debt_token_flash_loan_adaptor_call, get_balancer_pool_flash_loan_adaptor_call,
    Adaptors,
};
use crate::lifecycle::cork_vote::cork_voting_period;
use crate::{
    application::{self, get_channel},
    cellar_call::{
        construct_call_data, convert_to_aave_v3_flash_loan_adaptor,
        convert_to_balancer_pool_flash_loan_adaptor, CellarCallData,
    },
    lifecycle,
    state::{RequestState, RequestStatus},
};
use crate::cellar_call::{CellarCall, create_cellar_call};

pub(crate) fn validate(
    cellar_id: &str,
    block_height: u64,
    chain_id: u64,
    deadline: u64,
    calls: &Vec<CellarCallData>,
) -> Result<()> {
    if Address::from_str(cellar_id).is_err() {
        bail!("invalid cellar address");
    }

    if block_height == 0 {
        bail!("block height cannot be zero");
    }

    if chain_id == 0 {
        bail!("invalid chain id");
    }

    if deadline == 0 {
        bail!("deadline cannot be zero");
    }

    log::trace!("validating calls");

    if let Err(err) = validate_calls(calls) {
        log::error!("error validating calls: {:?}", err);
        return Err(err);
    }

    Ok(())
}

pub(crate) fn validate_calls(calls: &[CellarCallData]) -> Result<()> {
    if calls.is_empty() {
        bail!("cellar call data is empty");
    };

    for call in calls.iter() {
        if let Some(adaptor) = &call.adaptor_address {
            if adaptor.is_empty() {
                bail!("adaptor id is empty");
            }

            if Address::from_str(adaptor).is_err() {
                bail!("invalid adaptor address");
            }
        }

        if call.fields.is_empty() {
            bail!("cellar call fields are empty");
        }
    }

    Ok(())
}

pub(crate) fn build_request(
    cellar_id: String,
    block_height: u64,
    chain_id: u64,
    deadline: u64,
    queue: Vec<CellarCallData>,
) -> Result<ScheduleRequest, serde_json::Error> {
    let function_input = create_cellar_call(queue)?;
    let call_data = Some(construct_call_data(function_input));
    Ok(ScheduleRequest {
        cellar_id,
        chain_id,
        block_height,
        deadline,
        call_data,
    })
}

pub(crate) fn build_flash_loan_request(
    cellar_id: String,
    block_height: u64,
    chain_id: u64,
    deadline: u64,
    flash_loan_call: CellarCallData,
    queue: Vec<CellarCallData>,
) -> Result<ScheduleRequest> {
    let adaptor_calls: Vec<AdaptorCall> = queue
        .into_iter()
        .map(|call| call.to_adaptor_call())
        .collect::<Result<_>>()?;

    let adaptor_str = match &flash_loan_call.adaptor_address {
        Some(adaptor) => adaptor,
        None => bail!("Adaptor is None"),
    };

    let flash_loan_adaptor_call = match flash_loan_call.adaptor_name {
        Some(Adaptors::AaveV3DebtTokenV1FlashLoan) => {
            let adaptor_calls_for_flash_loan = adaptor_calls
                .into_iter()
                .map(|call| convert_to_aave_v3_flash_loan_adaptor(&call))
                .collect::<Result<Vec<AdaptorCallForAaveV3FlashLoan>>>()?;

            get_aave_v3_debt_token_flash_loan_adaptor_call(
                adaptor_str,
                &flash_loan_call.fields,
                adaptor_calls_for_flash_loan,
            )
        }
        Some(Adaptors::BalancerPoolV1FlashLoan) => {
            let adaptor_calls_for_flash_loan = adaptor_calls
                .into_iter()
                .map(|call| convert_to_balancer_pool_flash_loan_adaptor(&call))
                .collect::<Result<Vec<AdaptorCallForBalancerPoolFlashLoan>>>()?;

            get_balancer_pool_flash_loan_adaptor_call(
                adaptor_str,
                &flash_loan_call.fields,
                adaptor_calls_for_flash_loan,
            )
        }
        Some(_) => unreachable!("Unsupported flash loan variant encountered"),
        None => bail!("No adaptor name provided"),
    }?;


    let flash_loan_adaptor_call = vec![flash_loan_adaptor_call];

    let function = cellar_v2_5::function_call::Function::CallOnAdaptor(
        cellar_v2_5::CallOnAdaptor {
            data: flash_loan_adaptor_call

        }
    );

    let call_data = Some(construct_call_data(function));

    Ok(ScheduleRequest {
        cellar_id,
        chain_id,
        block_height,
        deadline,
        call_data,
    })
}

/// Handles submitting and tracking the request
pub(crate) async fn handle(app_handle: tauri::AppHandle, request: ScheduleRequest) -> Result<()> {
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
        broadcast_schedule_request(trace_id, app_handle.clone(), tx.clone(), request).await?;
    let (cork_id, _invalidation_scope) = match request_status {
        RequestStatus::FailedBroadcast => {
            log::warn!(id = trace_id; "broadcast failed");

            return Ok(());
        }
        RequestStatus::AwaitingVote((cork_id, invalidation_scope)) => (cork_id, invalidation_scope),
        _ => bail!("unexpected request status after broadcast: {request_status:?}"),
    };

    // Wait for the scheduled height
    let request_status = cork_voting_period(app_handle, &cork_id, height, tx.clone()).await?;
    let tx = match request_status {
        RequestStatus::AwaitingConfirmation => tx,
        RequestStatus::FailedVote => return Ok(()),
        _ => bail!("unexpected request status after voting period: {request_status:?}"),
    };

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
    app_handle: tauri::AppHandle,
    tx: Sender<RequestStatus>,
    request: ScheduleRequest,
) -> Result<RequestStatus> {
    log::trace!(id; "broadcasting schedule request");

    let app_context = app_handle.state::<application::Context>();

    let Some(subscribers) = app_context.0.read().await.subscribers.to_owned() else {
        log::error!(id; "no subscribers found");

        tx.send(RequestStatus::FailedBroadcast).await?;
        return Ok(RequestStatus::FailedBroadcast);
    };

    let Some(identity) = app_context.0.read().await.client_identity.clone() else {
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

    log::debug!("broadcast results: {result:?}");

    // Extract the cork ID from one of the successful responses.
    // If there were no successful responses this iterator has length zero and nothing happens,
    // resulting in a failed broadcast.
    if let Some((cid, is)) = result.into_iter().flatten().flatten().next() {
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
