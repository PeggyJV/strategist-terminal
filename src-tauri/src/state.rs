use std::{collections::HashMap, sync::Arc};

use steward_proto::proto::ScheduleRequest;
use tokio::sync::Mutex;
use uuid::Uuid;

/// Represents the state of all requests
#[derive(Clone, Debug, Default)]
pub struct Requests(pub Arc<Mutex<HashMap<Uuid, RequestState>>>);

impl Requests {
    /// Creates a new requests state
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(HashMap::new())))
    }

    /// Adds a request to the state
    pub async fn add(&self, request: RequestState) {
        self.0.lock().await.insert(request.id, request);
    }
}

/// Represents the state of a particular request
#[derive(Clone, Debug, Default)]
pub struct RequestState {
    /// Request ID
    pub id: Uuid,
    /// The tracked [`ScheduleRequest`]
    pub request: ScheduleRequest,
    /// The current state in the request lifecycle
    pub status: RequestStatus,
    /// Cork ID Hash
    pub cork_id: Option<String>,
    /// Gravity invalidation scope
    pub invalidate_scope: Option<String>,
    /// The Sommelier transaction hash for the IBC relay request.
    ///
    /// This is only relevant for non-ethereum chains.
    pub relay_request_tx_hash: String,
    /// Axelar GMP transaction hash
    pub gmp_tx_hash: Option<String>,
    /// Transaction hash on the target chain
    pub target_tx_hash: Option<String>,
}

impl RequestState {
    /// Creates a new request state with a random ID
    pub fn new() -> Self {
        let id = Uuid::new_v4();

        Self {
            id,
            ..Default::default()
        }
    }
}

/// Represents the status of the request in its lifecycle
#[derive(Clone, Debug, Default)]
pub enum RequestStatus {
    /// The request has been created but not broadcast
    #[default]
    Initialized,
    /// The request is being broadcast to subscribers
    Broadcasting,
    /// The request was unable to be broadcast
    FailedBroadcast,
    /// The request has been broadcast and the terminal is monitoring votes for a quorum.
    ///
    /// Contains the (Cork ID, Invalidation scope) tuple.
    AwaitingVote((String, String)),
    /// Not enough votes were received before the scheduled height.
    FailedVote,
    /// Ethereum only. The scheduled height has been reached, the vote passed. Waiting for a quorum of validators
    /// to sign the outgoing transaction.
    AwaitingConfirmation,
    /// Non-ethereum chains only. An IBC transaction has been submitted to Sommelier for relaying
    /// to the Axelar network.
    ///
    /// Contains the Sommelier transaction hash of the IBC tx.
    AwaitingRelay(String),
    /// Non-ethereum chains only. The transaction has been relayed from Sommelier to the Axelar network.
    ///
    /// Contains the Axelar GMP transaction hash.
    Relayed(String),
    /// The transaction has a quorum of singatures (Ethereum) or has been relayed to the Axelar network (Non-ethereum)
    /// and the terminal is monitoring the target contract for execution.
    AwaitingExecution,
    /// The transaction has been executed on the target chain and failed
    ///
    /// Contains the transaction hash on the target chain.
    FailedExecution(String),
    /// The transaction has been executed on the target chain and succeeded
    ///
    /// Contains the transaction hash on the target chain.
    Success(String),
    /// Unable to determine the state of the request
    Unknown,
}

/// Represents state info for the Sommelier chain
#[derive(Clone, Debug, Default)]
pub struct SommelierState {
    /// The current block height
    pub block_height: u64,
}

/// Wrapper of the state info for the Sommelier chain
#[derive(Clone, Debug, Default)]
pub struct Sommelier(pub Arc<Mutex<SommelierState>>);

impl Sommelier {
    /// Creates a new Sommelier state
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(SommelierState::default())))
    }
}
