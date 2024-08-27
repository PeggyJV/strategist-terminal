use eyre::Result;
use tauri::Manager;
use tendermint_rpc::{Client, HttpClient};

use crate::state;

/// Keeps state for the block height in sync with the chain
pub async fn sync_block_height(app_handle: tauri::AppHandle, rpc_endpoint: String) -> Result<()> {
    loop {
        if let Err(err) = refresh_block_height(app_handle.clone(), &rpc_endpoint).await {
            log::error!("failed to refresh block height: {}", err);
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}

/// Retrieves the latest block height using a Tendermint RPC client
async fn refresh_block_height(app_handle: tauri::AppHandle, rpc_endpoint: &str) -> Result<()> {
    log::debug!("getting latest block height");

    let client = loop {
        match HttpClient::new(rpc_endpoint) {
            Ok(client) => break client,
            Err(err) => {
                log::error!("failed to create RPC client: {}", err);
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        }
    };

    let block_height = client.abci_info().await?.last_block_height.value();
    let sommelier_state = app_handle.state::<state::Sommelier>();
    let mut sommelier_state = sommelier_state.0.lock().await;

    sommelier_state.block_height = block_height;

    log::debug!("block height: {}", block_height);

    Ok(())
}
