use std::str::FromStr;

use alloy_primitives::Address;
use eyre::{bail, Result};
use serde::{Deserialize, Serialize};
use somm_proto::pubsub::Subscriber;
use steward_proto::proto::{
    cellar_v2_5::{function_call, CallOnAdaptor, CallType, FunctionCall},
    contract_call_service_client::ContractCallServiceClient,
    schedule_request::CallData,
    AdaptorCall, CellarV25, ScheduleRequest, ScheduleResponse,
};
use tonic::transport::{Channel, Identity};
use tracing::{debug, error, info};

use crate::app::{self, get_channel, AppContext};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub(crate) struct RequestData {
    pub adaptor_id: String,
    pub chain_id: u64,
    pub cellar_id: String,
    pub block_height: u64,
    pub deadline: u64,
    pub adaptor_call: AdaptorCall,
}

pub(crate) fn validate_data(data: &Option<RequestData>) -> Result<()> {
    let Some(data) = data else {
        bail!("request data is empty");
    };

    if data.adaptor_id.is_empty() {
        bail!("adaptor id is empty");
    }

    if Address::from_str(&data.adaptor_id).is_err() {
        bail!("invalid adaptor address");
    }

    if Address::from_str(&data.cellar_id).is_err() {
        bail!("invalid cellar address");
    }

    if data.chain_id == 0 {
        bail!("invalid chain id");
    }

    Ok(())
}

pub(crate) fn build_request(data: RequestData) -> Result<ScheduleRequest> {
    let call_data = Some(CallData::CellarV25(CellarV25 {
        call_type: Some(CallType::FunctionCall(FunctionCall {
            function: Some(function_call::Function::CallOnAdaptor(CallOnAdaptor {
                data: vec![data.adaptor_call],
            })),
        })),
    }));

    let request = ScheduleRequest {
        cellar_id: data.cellar_id,
        chain_id: data.chain_id,
        block_height: data.block_height,
        deadline: data.deadline,
        call_data,
    };

    Ok(request)
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

    // TODO: concurrently execute each request and capture all results
    for subscriber in subscribers.into_iter() {
        debug!(
            "sending contract call to subscriber {}",
            &subscriber.push_url
        );
        let request = request.clone();
        let Some(identity) = context.client_identity.clone() else {
            bail!("app context does not contain client identity. user must provide their certificate and key.");
        };

        match handle_scheduling(identity, subscriber, request).await {
            Ok(res) => info!("response: {res:?}"),
            Err(err) => error!("error: {err:?}"),
        }
    }

    Ok(())
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
        let valid_data = RequestData {
            adaptor_id: "0xf9d0bb4fE3a004bE6766005EE9Fb889A8A0DCED3".to_string(),
            chain_id: 42161,
            cellar_id: "0xf9d0bb4fE3a004bE6766005EE9Fb889A8A0DCED3".to_string(),
            block_height: 12345689,
            deadline: 12345689101112,
            adaptor_call: AdaptorCall::default(),
        };
        let invalid_data1 = RequestData {
            adaptor_id: "".to_string(),
            chain_id: 42161,
            cellar_id: "0xf9d0bb4fE3a004bE6766005EE9Fb889A8A0DCED3".to_string(),
            block_height: 12345689,
            deadline: 12345689101112,
            adaptor_call: AdaptorCall::default(),
        };
        let invalid_data2 = RequestData {
            adaptor_id: "0xf9d0bb4fE3a004bE6766005EE9Fb889A8A0DCED3".to_string(),
            chain_id: 0,
            cellar_id: "0xf9d0bb4fE3a004bE6766005EE9Fb889A8A0DCED3".to_string(),
            block_height: 12345689,
            deadline: 12345689101112,
            adaptor_call: AdaptorCall::default(),
        };
        let invalid_data3 = RequestData {
            adaptor_id: "0xf9d0bb4fE3a004bE6766005EE9Fb889A8A0DCED3".to_string(),
            chain_id: 42161,
            cellar_id: "".to_string(),
            block_height: 12345689,
            deadline: 12345689101112,
            adaptor_call: AdaptorCall::default(),
        };

        assert!(validate_data(&Some(valid_data.clone())).is_ok());
        let result = validate_data(&Some(invalid_data1.clone()));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "adaptor id is empty");
        let result = validate_data(&Some(invalid_data2.clone()));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "invalid chain id");
        let result = validate_data(&Some(invalid_data3.clone()));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "invalid cellar address");
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
