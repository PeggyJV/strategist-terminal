use std::str::FromStr;

use alloy_primitives::Address;
use eyre::{bail, Result};
use futures::StreamExt;
use serde::Deserialize;
use somm_proto::pubsub::Subscriber;
use steward_proto::proto::{
    contract_call_service_client::ContractCallServiceClient, ScheduleRequest, ScheduleResponse,
};
use tonic::transport::{Channel, Identity};
use tracing::{debug, error, info};

use crate::{
    app::{self, get_channel, AppContext},
    cellar_call::{construct_call_data, CellarCall},
};

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub(crate) struct ScheduleRequestData {
    pub cellar_id: String,
    pub block_height: u64,
    pub chain_id: u64,
    pub deadline: u64,
    pub queue: Vec<CellarCall>,
}

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

pub(crate) fn handle(request: ScheduleRequest) {
    futures::executor::block_on(async move {
        let app_context = app::get_app_context().await;

        broadcast_schedule_request(&app_context, request)
            .await
            .unwrap_or_else(|err| {
                // TODO: find a way to bubble up errors to tauri command
                error!("{err:?}");
            });
    });
}

/// Broadcasts the [`ScheduleRequest`] to all subscribers
async fn broadcast_schedule_request(context: &AppContext, request: ScheduleRequest) -> Result<()> {
    let Some(subscribers) = context.subscribers.to_owned() else {
        bail!("subscribers cache has not been initialized")
    };

    let Some(identity) = context.client_identity.clone() else {
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

    posts.await;

    Ok(())
}

async fn handle_subscriber(
    identity: Identity,
    subscriber: Subscriber,
    request: ScheduleRequest,
) -> Result<()> {
    debug!(
        "sending contract call to subscriber {}",
        &subscriber.push_url
    );

    let request = request.clone();

    handle_scheduling(identity, subscriber, request).await
}

async fn handle_scheduling(
    identity: Identity,
    subscriber: Subscriber,
    request: ScheduleRequest,
) -> Result<()> {
    let channel = get_channel(identity, subscriber.ca_cert, subscriber.push_url.clone()).await?;

    info!("sending schedule call to {}", subscriber.push_url);
    match schedule(channel, request).await {
        Ok(res) => {
            debug!("response from {}: {res:?}", subscriber.push_url);
        }
        Err(err) => error!(
            "failed to send contract call to subscriber {}: {err:?}",
            subscriber.address
        ),
    }

    Ok(())
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
    async fn test_validate_data() {
        let valid_data = ScheduleRequestData {
            chain_id: 42161,
            cellar_id: "0xf9d0bb4fE3a004bE6766005EE9Fb889A8A0DCED3".to_string(),
            block_height: 12345689,
            deadline: 12345689101112,
            queue: vec![CellarCall::default()],
        };
        let invalid_data1 = ScheduleRequestData {
            chain_id: 42161,
            cellar_id: "0xf9d0bb4fE3a004bE6766005EE9Fb889A8A0DCED3".to_string(),
            block_height: 12345689,
            deadline: 12345689101112,
            queue: vec![CellarCall::default()],
        };
        let invalid_data2 = ScheduleRequestData {
            chain_id: 0,
            cellar_id: "0xf9d0bb4fE3a004bE6766005EE9Fb889A8A0DCED3".to_string(),
            block_height: 12345689,
            deadline: 12345689101112,
            queue: vec![CellarCall::default()],
        };
        let invalid_data3 = ScheduleRequestData {
            chain_id: 42161,
            cellar_id: "".to_string(),
            block_height: 12345689,
            deadline: 12345689101112,
            queue: vec![CellarCall::default()],
        };
    }

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

        set.spawn(handle_scheduling(
            identity.clone(),
            subscriber.clone(),
            request.clone(),
        ));
        set.spawn(handle_scheduling(
            identity.clone(),
            subscriber.clone(),
            request.clone(),
        ));
        set.spawn(handle_scheduling(
            identity.clone(),
            subscriber.clone(),
            request.clone(),
        ));

        while let Some(res) = set.join_next().await {
            info!("result: {res:?}");
        }
    }
}
