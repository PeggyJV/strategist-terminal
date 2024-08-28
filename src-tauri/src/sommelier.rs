use eyre::Result;
use log::debug;
use once_cell::sync::OnceCell;
use tauri::Manager;
use tendermint_rpc::{Client, HttpClient};

use crate::state;

const HEIGHT_REFRESH_INTERVAL: tokio::time::Duration = tokio::time::Duration::from_secs(5);

static HTTP_CLIENT: OnceCell<HttpClient> = OnceCell::new();

async fn get_or_init_client(rpc_endpoint: &str) -> Result<&'static HttpClient> {
    HTTP_CLIENT
        .get_or_try_init(|| HttpClient::new(rpc_endpoint))
        .map_err(Into::into)
}

/// Keeps state for the block height in sync with the chain
pub async fn sync_block_height(app_handle: tauri::AppHandle, rpc_endpoint: String) -> Result<()> {
    loop {
        if let Err(err) = refresh_block_height(app_handle.clone(), &rpc_endpoint).await {
            log::error!("failed to refresh block height: {}", err);
        }

        tokio::time::sleep(HEIGHT_REFRESH_INTERVAL).await;
    }
}

/// Retrieves the latest block height using a Tendermint RPC client
async fn refresh_block_height(app_handle: tauri::AppHandle, rpc_endpoint: &str) -> Result<()> {
    debug!("getting latest block height");

    let client = get_or_init_client(rpc_endpoint).await?;

    let block_height = client.abci_info().await?.last_block_height.value();
    let sommelier_state = app_handle.state::<state::Sommelier>();
    let mut sommelier_state = sommelier_state.0.lock().await;

    sommelier_state.block_height = block_height;

    debug!("block height: {}", block_height);

    Ok(())
}
