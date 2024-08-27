use alloy::hex;
use eyre::{bail, Result};
use tauri::async_runtime::Sender;

use crate::state::RequestStatus;

use std::str::FromStr;

use alloy::{
    providers::{Provider, ProviderBuilder},
    sol,
};
use alloy_primitives::{FixedBytes, U256};

const GRAVITY_CONTRACT_ADDRESS: &str = "0x69592e6f9d21989a043646fe8225da2600e5a0f7";

sol!(
    #[sol(rpc)]
    Gravity,
    "abi/GravityABI.json",
);

/// Monitors the Gravity contract for logic call execution and update the request status
pub(crate) async fn watch(
    id: &str,
    tx: Sender<RequestStatus>,
    invalidation_scope: &str,
    token: tokio_util::sync::CancellationToken,
) -> Result<()> {
    log::trace!(id, invalidation_scope; "watching for Gravity contract execution");

    // Get the latest invalidation nonce for the scope from the Gravity contract
    let rpc_url = "https://eth.merkle.io".parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);

    let gravity_contract = Gravity::new(GRAVITY_CONTRACT_ADDRESS.parse()?, provider.clone());
    let invalidation_scope_bytes = FixedBytes::<32>::from_str(&invalidation_scope)?;

    let result = gravity_contract
        .lastLogicCallNonce(invalidation_scope_bytes)
        .call()
        .await?;
    let last_invalidation_nonce = result._0;

    log::trace!(id, invalidation_scope, last_invalidation_nonce = last_invalidation_nonce.to_string().as_str(); "got last invalidation nonce");

    // Monitor the Gravity contract for LogicCallEvent with the invalidation scope and higher invalidation
    // nonce than the last one executed
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(10));

    let mut nonce = U256::ZERO;
    let transaction_hash: Result<Option<FixedBytes<32>>, eyre::ErrReport> = tokio::time::timeout(tokio::time::Duration::from_secs(300), async {
        let mut transaction_hash = None;
        let provider = provider.clone();

        loop {
            tokio::select! {
                _ = token.cancelled() => {
                    log::trace!(id, invalidation_scope; "gravity monitor cancelled");
                    break;
                },
                _ =  interval.tick() => {
                    log::trace!(id, invalidation_scope; "querying latest block");

                    // Check logs from the latest block for LogicCallEvents
                    let latest_block = provider.get_block_number().await?;

                    log::trace!(id, invalidation_scope, latest_block; "querying Gravity logs for logic call event");

                    let event_logs = gravity_contract.LogicCallEvent_filter().from_block(latest_block).query().await?;

                    log::trace!(id, invalidation_scope, latest_block, total_logs = event_logs.len(); "got Gravity {} logs", event_logs.len());

                    for (event, log) in event_logs {
                        if event._invalidationId.eq(&invalidation_scope_bytes) && event._invalidationNonce.gt(&(last_invalidation_nonce)) {
                            log::info!(id, invalidation_scope, invalidation_nonce = event._invalidationNonce.to_string().as_str(); "found logic call event");
                            transaction_hash = log.transaction_hash;
                            nonce = event._invalidationNonce;

                            break;
                        }
                    }
                },
            };
        }

        Ok(transaction_hash)
    }).await?;

    let transaction_hash = transaction_hash?;

    // Determine if execution succeeded
    if let Some(transaction_hash) = transaction_hash {
        let hash = hex::encode(transaction_hash);
        let invalidation_nonce = nonce.to_string();
        let invalidation_nonce = invalidation_nonce.as_str();
        log::info!(id, invalidation_scope, invalidation_nonce, transaction_hash = hash.as_str(); "found logic call event");

        let Some(receipt) = provider.get_transaction_receipt(transaction_hash).await? else {
            log::warn!(id, invalidation_scope, invalidation_nonce, transaction_hash = hash.as_str(); "couldn't get transaction receipt");
            bail!("transaction receipt was empty");
        };

        if receipt.status() {
            log::info!(id, invalidation_scope, invalidation_nonce, transaction_hash = hash.as_str(); "execution success");

            tx.send(RequestStatus::Success(hash.clone())).await?;
        } else {
            log::info!(id, invalidation_scope, invalidation_nonce, transaction_hash = hash.as_str(); "execution failed");

            tx.send(RequestStatus::FailedExecution(hash.clone()))
                .await?;
        }
    }

    // Since this will run in the background throughout a request's lifecycle, we don't update the
    // status if we can't find the transaction hash, and we leave it to other threads to determine if
    // the request is in an unknown state.
    Ok(())
}
