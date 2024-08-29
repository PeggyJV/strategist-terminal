use std::str::FromStr;

use alloy::{
    contract::Event,
    primitives::address,
    providers::{Provider, ProviderBuilder},
    rpc::types::Filter,
    sol,
};
use alloy_primitives::FixedBytes;
use eyre::Result;
use tauri::async_runtime::Sender;
use toml::from_str;

use crate::{application::AppContext, state::RequestStatus};

const GRAVITY_CONTRACT_ADDRESS: &str = "0x69592e6f9d21989a043646fe8225da2600e5a0f7";

sol!(
    #[sol(rpc)]
    Gravity,
    "abi/GravityABI.json",
);

//pub(crate) async fn awaiting_relay(app_context: &AppContext, tx: Sender<RequestStatus>, invalidation_scope: &str) -> Result<RequestStatus> {
//    // Monitor Gravity contract for LogicCallEvent using the invalidation scope as the
//    // invalidationId
//    let rpc_url = "https://eth.merkle.io".parse()?;
//    let provider = ProviderBuilder::new().on_http(rpc_url);
//
//    // Get logs from the latest block
//    let latest_block = provider.get_block_number().await?;
//
//    let gravity_contract = Gravity::new(GRAVITY_CONTRACT_ADDRESS.parse()?, provider);
//
//    let event_logs = gravity_contract.LogicCallEvent_filter().from_block(latest_block).query().await?;
//
//    let invalidation_scope_bytes = FixedBytes::<32>::from_str(&invalidation_scope)?;
//
//    for (event, log) in event_logs {
//        if event._invalidationId.eq(&invalidation_scope_bytes) {
//            let status = RequestStatus::Relayed;
//            tx.send(status).await?;
//            return Ok(status);
//        }
//    }
//
//    Ok(())
//}
