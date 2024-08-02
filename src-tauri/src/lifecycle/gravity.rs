use alloy::{hex, providers::RootProvider};
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
    tx: Sender<RequestStatus>,
    invalidation_scope: &str,
    token: tokio_util::sync::CancellationToken,
) -> Result<()> {
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

    // Monitor the Gravity contract for LogicCallEvent with the invalidation scope and higher invalidation
    // nonce than the last one executed
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(10));

    let transaction_hash: Result<Option<FixedBytes<32>>, eyre::ErrReport> = tokio::time::timeout(tokio::time::Duration::from_secs(300), async {
        let mut transaction_hash = None;
        let provider = provider.clone();

        loop {
            tokio::select! {
                _ = token.cancelled() => {
                    break;
                },
                _ =  interval.tick() => {
                    // Check logs from the latest block for LogicCallEvents
                    let latest_block = provider.get_block_number().await?;
                    let event_logs = gravity_contract.LogicCallEvent_filter().from_block(latest_block).query().await?;

                    for (event, log) in event_logs {
                        if event._invalidationId.eq(&invalidation_scope_bytes) && event._invalidationNonce.gt(&(last_invalidation_nonce)) {
                            transaction_hash = log.transaction_hash;

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
        let Some(receipt) = provider.get_transaction_receipt(transaction_hash).await? else {
            bail!("transaction receipt was empty");
        };

        let hash = hex::encode(transaction_hash);

        if receipt.status() {
            tx.send(RequestStatus::Success(hash.clone())).await?;
        } else {
            tx.send(RequestStatus::FailedExecution(hash.clone()))
                .await?;
        }
    }

    // Since this will run in the background throughout a request's lifecycle, we don't update the
    // status if we can't find the transaction hash, and we leave it to other threads to determine if
    // the request is in an unknown state.
    Ok(())
}
