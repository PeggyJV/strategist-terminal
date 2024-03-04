use eyre::{bail, Result};
use somm_proto::pubsub::Subscriber;
use steward_proto::proto::{
    status_service_client::StatusServiceClient, VersionRequest, VersionResponse,
};
use tonic::transport::{Channel, Identity};
use tracing::{debug, error, info};

use crate::app::{self, get_channel, AppContext};

pub(crate) fn handle() {
    futures::executor::block_on(async move {
        let app_context = app::get_app_context().await;

        broadcast_version_request(&app_context)
            .await
            .unwrap_or_else(|err| {
                // TODO: find a way to bubble up errors to tauri command
                error!("{err:?}");
            });
    });
}

/// Broadcasts a [`VersionRequest`] to all subscribers
async fn broadcast_version_request(context: &AppContext) -> Result<()> {
    let Some(subscribers) = context.subscribers.to_owned() else {
        bail!("subscribers cache has not been initialized")
    };

    // TODO: concurrently execute each request and capture all results
    for subscriber in subscribers.into_iter() {
        let Some(identity) = context.client_identity.clone() else {
            bail!("app context does not contain client identity. user must provide their certificate and key.");
        };

        match handle_version(identity, subscriber).await {
            Ok(res) => info!("response: {res:?}"),
            Err(err) => error!("error: {err:?}"),
        }
    }

    Ok(())
}

async fn handle_version(identity: Identity, subscriber: Subscriber) -> Result<()> {
    let channel = get_channel(identity, subscriber.ca_cert, subscriber.push_url.clone()).await?;

    debug!("sending version request to {}", subscriber.push_url);
    match version(channel).await {
        Ok(res) => {
            debug!("response from {}: {res:?}", subscriber.push_url);
        }
        Err(err) => error!(
            "failed to get steward version for subscriber {}: {err:?}",
            subscriber.address
        ),
    }

    Ok(())
}

async fn version(channel: Channel) -> Result<tonic::Response<VersionResponse>> {
    let mut client = StatusServiceClient::new(channel);

    let request = VersionRequest {};
    client.version(request).await.map_err(Into::into)
}
