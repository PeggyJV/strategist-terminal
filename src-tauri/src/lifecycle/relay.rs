use std::str::FromStr;

use eyre::Result;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use tauri::async_runtime::Sender;
use tendermint_rpc::query::{EventType, Query};
use tendermint_rpc::{Client, HttpClient, Order, Response, SubscriptionClient, WebSocketClient};

use crate::state::RequestStatus;

pub(crate) async fn monitor_ibc_relay(
    app_handle: tauri::AppHandle,
    trace_id: String,
    tx: Sender<RequestStatus>,
    chain_id: u64,
    cellar_id: String,
    vote_height: u64,
) -> Result<RequestStatus> {
    log::info!(id = trace_id, chain_id, cellar_id; "monitoring IBC relay");

    // TODO: This needs to be a long running poll since the relay request can come in at any time

    let rpc_url = "https://sommelier-rpc.polkachu.com:443"; // Replace with your WebSocket URL
    let client = HttpClient::new(rpc_url)?;

    // Construct the query
    let query = format!(
        "transfer.sender='somm1lrneqhq4rq8nz2nk6vn3sanrxva7zuns8aa45g' AND tx.height>{vote_height}"
    );
    let query = Query::from_str(&query)?;
    let mut response = client
        .tx_search(query.clone(), false, 1, 1, Order::Ascending)
        .await?;
    let mut attempts = 0;
    let max_attempts = 10; // Adjust this value as needed
    let delay = std::time::Duration::from_secs(5); // 5 seconds delay between attempts

    while response.txs.is_empty() {
        if attempts == max_attempts {
            log::warn!(id = trace_id, chain_id, cellar_id; "No transactions found after {} attempts", max_attempts);
            return Ok(RequestStatus::Unknown);
        }
        tokio::time::sleep(delay).await;
        response = client
            .tx_search(query.clone(), false, 1, 10, Order::Ascending)
            .await?;
        attempts += 1;
    }

    // Temporary
    // Convert response to JSON and write to file
    let json_response = serde_json::to_string_pretty(&response)?;
    let file_name = format!("tx_search_response_{}.json", chrono::Utc::now().timestamp());
    std::fs::write(&file_name, json_response)?;
    log::info!(id = trace_id, chain_id, cellar_id, file_name; "Wrote tx_search response to file");

    // Find the transaction that is a relay request
    let transaction = response.txs.iter().find(|response| {
        response.tx_result.events.iter().any(|event| {
            event.kind == "message"
                && event.attributes.iter().any(|attr| {
                    // base64 encoding of "action"
                    attr.key_str().unwrap() == "YWN0aW9u"
                        // base64 encoding of "/axelarcork.v1.MsgRelayAxelarCorkRequest"
                        && attr.value_str().unwrap() == "L2F4ZWxhcmNvcmsudjEuTXNnUmVsYXlBeGVsYXJDb3JrUmVxdWVzdA=="
                })
        })
    });

    let Some(transaction) = transaction else {
        log::info!(id = trace_id, chain_id, cellar_id; "no axelar cork relay request found");
        return Ok(RequestStatus::Unknown);
    };

    let tx_hash = transaction.hash.clone();
    let events = transaction.tx_result.events.clone();
    log::debug!(id = trace_id, chain_id, cellar_id, tx_hash = tx_hash.to_string(); "transaction found");

    // Search for the send_packet event and extract the destination address from the memo
    let found = events.iter().any(|event| {
        use base64::{engine::general_purpose, Engine as _};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct PacketData {
            amount: String,
            denom: String,
            memo: String,
            sender: String,
            receiver: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct AxelarMemo {
            destination_chain: String,
            destination_address: String,
            payload: String,
            #[serde(rename = "type")]
            memo_type: u32,
            fee: Fee,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct Fee {
            amount: String,
            recipient: String,
        }

        // TODO: Since the relay request can come at any time, it's probably not sufficient to query based on the sender address and the height. See if there is some info that can be extracted from the events or the memo that would narrow down the result to only the relevant transaction. It may require subscribing to the event instead of querying (push model instead of pull).
        event.kind == "send_packet"
            && event.attributes.iter().any(|attr| {
                let decoded_key = general_purpose::STANDARD
                    .decode(attr.key_str().unwrap())
                    .unwrap();
                let decoded_key = String::from_utf8_lossy(&decoded_key);
                let decoded_value = general_purpose::STANDARD
                    .decode(attr.value_str().unwrap())
                    .unwrap();
                let decoded_value = String::from_utf8_lossy(&decoded_value);

                let memo = if decoded_key == "packet_data" {
                    let packet_data: PacketData = serde_json::from_str(&decoded_value).unwrap();

                    serde_json::from_str::<AxelarMemo>(&packet_data.memo).unwrap()
                } else {
                    return false;
                };

                memo.destination_chain == chain_id.to_string()
                    && memo.destination_address == cellar_id
            })
    });

    if found {
        log::info!(id = trace_id, chain_id, cellar_id, tx_hash = tx_hash.to_string(); "MsgRelayAxelarCork request found. Awaiting relay.");
        tx.send(RequestStatus::AwaitingRelay(tx_hash.to_string()))
            .await?;
        return Ok(RequestStatus::AwaitingRelay(tx_hash.to_string()));
    }

    Ok(RequestStatus::Unknown)
}
