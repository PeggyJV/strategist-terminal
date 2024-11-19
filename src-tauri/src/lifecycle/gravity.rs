use alloy::hex;
use eyre::{bail, Result};
use steward_proto::proto::ScheduleRequest;
use tauri::async_runtime::Sender;

use crate::state::RequestStatus;

use std::str::FromStr;

use alloy::{
    providers::{Provider, ProviderBuilder},
    sol,
};
use alloy_primitives::{FixedBytes, U256};

use super::cork_vote::{monitor_cork_voting_period, CorkType};

const GRAVITY_CONTRACT_ADDRESS: &str = "0x69592e6f9d21989a043646fe8225da2600e5a0f7";
// const GRAVITY_CONTRACT_ADDRESS: &str = "0xDc64a140Aa3E981100a9becA4E685f962f0cF6C9";

sol!(
    #[sol(rpc)]
    Gravity,
    "abi/Gravity.json",
);

pub(crate) async fn track_gravity_cork(
    app_handle: tauri::AppHandle,
    trace_id: String,
    tx: Sender<RequestStatus>,
    request: ScheduleRequest,
    request_status: RequestStatus,
) -> Result<()> {
    let (cork_id, invalidation_scope) = match request_status {
        RequestStatus::FailedBroadcast => {
            log::warn!(id = trace_id; "broadcast failed");

            return Ok(());
        }
        RequestStatus::AwaitingVote((cork_id, invalidation_scope)) => (cork_id, invalidation_scope),
        _ => bail!("unexpected request status after broadcast: {request_status:?}"),
    };

    // Watch the Gravity contract for the logic call execution
    tokio::spawn(monitor_gravity_logic_call_execution(
        trace_id.to_string(),
        tx.clone(),
        invalidation_scope.clone(),
    ));

    // Wait for the scheduled height
    let request_status = monitor_cork_voting_period(
        app_handle,
        trace_id,
        CorkType::Gravity(cork_id),
        request.block_height,
        tx.clone(),
    )
    .await?;
    let tx = match request_status {
        RequestStatus::AwaitingConfirmation => tx,
        RequestStatus::FailedVote => return Ok(()),
        _ => bail!("unexpected request status after voting period: {request_status:?}"),
    };

    Ok(())
}

/// Monitors the Gravity contract for logic call execution and updates the request status
pub(crate) async fn monitor_gravity_logic_call_execution(
    id: String,
    tx: Sender<RequestStatus>,
    invalidation_scope: String,
) -> Result<()> {
    log::trace!(id, invalidation_scope; "watching for Gravity contract execution");

    // Get the latest invalidation nonce for the scope from the Gravity contract
    let rpc_url = match "https://arb-mainnet.g.alchemy.com/v2/nRe2NCR-6Ti4W-gHWCvE-fARS_w6HB9Y"
        .parse::<reqwest::Url>()
    {
        Ok(url) => url,
        Err(err) => {
            log::error!("error parsing Ethereum RPC URL: {err}");
            return Err(err.into());
        }
    };
    let provider = ProviderBuilder::new().on_http(rpc_url);

    let gravity_contract_address = match GRAVITY_CONTRACT_ADDRESS.parse() {
        Ok(address) => address,
        Err(err) => {
            log::error!("Failed to parse Gravity contract address: {}", err);
            return Err(eyre::eyre!(
                "Failed to parse Gravity contract address: {}",
                err
            ));
        }
    };
    let gravity_contract = Gravity::new(gravity_contract_address, provider.clone());
    let invalidation_scope_bytes = match FixedBytes::<32>::from_str(&invalidation_scope) {
        Ok(bytes) => bytes,
        Err(err) => {
            log::error!("Failed to parse invalidation scope: {}", err);
            return Err(eyre::eyre!("Failed to parse invalidation scope: {}", err));
        }
    };

    log::debug!(id, invalidation_scope, invalidation_scope_bytes = hex::encode(invalidation_scope_bytes.as_slice()); "parsed invalidation scope as bytes");

    let call = gravity_contract.lastLogicCallNonce(invalidation_scope_bytes);

    log::debug!(
        "lastLogicCallNonce call selector: {:?}",
        &call.calldata()[..4]
    );

    let result = match call.call().await {
        Ok(res) => res,
        Err(err) => {
            log::error!("Failed to get lastLogicCallNonce: {}", err);
            return Err(err.into());
        }
    };
    let last_invalidation_nonce = result._0;

    log::trace!(id, invalidation_scope, last_invalidation_nonce = last_invalidation_nonce.to_string().as_str(); "got last invalidation nonce");

    // Monitor the Gravity contract for LogicCallEvent with the invalidation scope and higher invalidation
    // nonce than the last one executed
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(10));

    // Right now we're not actually getting the nonce so this is always zero
    let mut nonce = U256::ZERO;
    let transaction_hash: Result<Option<FixedBytes<32>>, eyre::ErrReport> = tokio::time::timeout(tokio::time::Duration::from_secs(300), async {
        let mut transaction_hash = None;
        let provider = provider.clone();
        let mut lower_bound_block = match tokio::time::timeout(
            tokio::time::Duration::from_secs(60),
            async {
                loop {
                    match provider.get_block_number().await {
                        Ok(block) => return block,
                        Err(err) => {
                            log::warn!("Failed to get block number: {}. Retrying...", err);
                            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                            continue;
                        }
                    }
                }
            }
        ).await {
            Ok(block) => block,
            Err(err) => {
                log::error!("Timeout while trying to get block number: {}", err);
                return Err(eyre::eyre!("Timeout while trying to get block number: {}", err));
            }
        };

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    log::trace!(id, invalidation_scope; "querying latest block");

                    let latest_block = provider.get_block_number().await?;

                    log::trace!(id, invalidation_scope, lower_bound_block, latest_block; "querying blocks for transactions containing invalidation scope");

                    for block_number in lower_bound_block..=latest_block {
                        let block = provider.get_block(block_number.into(), alloy::rpc::types::BlockTransactionsKind::Full).await?;
                        if let Some(block) = block {
                            for tx in block.transactions.as_transactions().unwrap() {
                                if tx.input.as_ref().windows(invalidation_scope_bytes.len()).any(|window| window == invalidation_scope_bytes.as_slice()) {
                                    log::info!(id, invalidation_scope; "found transaction containing invalidation scope");
                                    transaction_hash = Some(tx.hash);
                                    break;
                                }
                            }
                        }
                        if transaction_hash.is_some() {
                            break;
                        }
                    }

                    if transaction_hash.is_some() {
                        break;
                    }

                    lower_bound_block = latest_block;
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

        let receipt = match provider.get_transaction_receipt(transaction_hash).await {
            Ok(receipt) => receipt,
            Err(err) => {
                log::error!(id, invalidation_scope, invalidation_nonce, transaction_hash = hash.as_str(), error:% = err; "error getting transaction receipt");
                return Err(err.into());
            }
        };
        let Some(receipt) = receipt else {
            log::warn!(id, invalidation_scope, invalidation_nonce, transaction_hash = hash.as_str(); "couldn't get transaction receipt");
            bail!("transaction receipt was empty");
        };

        if receipt.status() {
            log::info!(id, invalidation_scope, invalidation_nonce, transaction_hash = hash.as_str(); "execution success");

            tx.send(RequestStatus::Success(hash.clone())).await?;
        } else {
            log::info!(id, invalidation_scope, invalidation_nonce, transaction_hash = hash.as_str(); "execution failed");

            let mut retry_count = 0;
            let max_retries = 3;
            let timeout = std::time::Duration::from_secs(5);

            while retry_count < max_retries {
                match tokio::time::timeout(
                    timeout,
                    tx.send(RequestStatus::FailedExecution(hash.clone())),
                )
                .await
                {
                    Ok(Ok(_)) => break,
                    Ok(Err(err)) => {
                        log::error!(id, invalidation_scope, invalidation_nonce, transaction_hash = hash.as_str(), error:% = err, retry_count; "error sending failed execution status");
                        retry_count += 1;
                    }
                    Err(_) => {
                        log::warn!(id, invalidation_scope, invalidation_nonce, transaction_hash = hash.as_str(), retry_count; "timeout sending failed execution status");
                        retry_count += 1;
                    }
                }
            }

            if retry_count == max_retries {
                log::error!(id, invalidation_scope, invalidation_nonce, transaction_hash = hash.as_str(); "failed to send failed execution status after max retries");
            }
        }
    }

    // Since this will run in the background throughout a request's lifecycle, we don't update the
    // status if we can't find the transaction hash, and we leave it to other threads to determine if
    // the request is in an unknown state.
    Ok(())
}
