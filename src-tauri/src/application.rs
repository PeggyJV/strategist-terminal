#![allow(dead_code)]
use std::{fs, sync::Arc};

use eyre::{bail, Result};
use somm_proto::pubsub::Subscriber;
use tauri::Manager;
use tokio::sync::RwLock;
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Identity};
use tracing::debug;

use crate::config::AppConfig;

const DEFAULT_RPC_ENDPOINT: &str = "https://sommelier-rpc.polkachu.com:443";
const DEFAULT_GRPC_ENDPOINT: &str = "https://sommelier-grpc.polkachu.com:14190";

pub(crate) struct Context(pub(crate) Arc<RwLock<AppContext>>);

impl Context {
    pub(crate) fn new() -> Self {
        Self(Arc::new(RwLock::new(AppContext::default())))
    }
}

#[derive(Default)]
pub(crate) struct AppContext {
    pub rpc_endpoint: String,
    pub grpc_endpoint: String,
    pub publisher_domain: Option<String>,
    pub client_identity: Option<Identity>,
    // TODO: cache subscribers on disk
    pub subscribers: Option<Vec<Subscriber>>,
}

pub(crate) async fn apply_config(app_handle: tauri::AppHandle, config: AppConfig) {
    let client_identity = match (config.client_cert_path, config.client_cert_key_path) {
        (Some(cert_path), Some(key_path)) => {
            match get_publisher_identity(cert_path.clone(), key_path.clone()) {
                Ok(identity) => Some(identity),
                Err(e) => {
                    log::error!(
                        cert_path,
                        key_path;
                        "failed to get publisher identity: {e}"
                    );
                    None
                }
            }
        }
        _ => None,
    };

    log::debug!("writing to app context");
    let app_context = app_handle.state::<Context>();
    let mut lock = app_context.0.write().await;

    lock.rpc_endpoint = config
        .rpc_endpoint
        .unwrap_or(DEFAULT_RPC_ENDPOINT.to_string());
    lock.grpc_endpoint = config
        .grpc_endpoint
        .unwrap_or(DEFAULT_GRPC_ENDPOINT.to_string());

    if config.publisher_domain.is_some() && lock.publisher_domain.ne(&config.publisher_domain) {
        lock.publisher_domain = config.publisher_domain;
    }

    if client_identity.is_some() {
        lock.client_identity = client_identity;
    }

    drop(lock);
    log::debug!("wrote to app context");
}

fn get_publisher_identity(cert_path: String, cert_key_path: String) -> Result<Identity> {
    let client_cert = fs::read_to_string(cert_path)?;
    let client_key = fs::read_to_string(cert_key_path)?;

    Ok(Identity::from_pem(client_cert, client_key))
}

pub(crate) async fn get_subscribers(app_handle: tauri::AppHandle) -> Result<Vec<Subscriber>> {
    let app_context = app_handle.state::<Context>();
    let grpc_endpoint = app_context.0.read().await.grpc_endpoint.clone();
    let mut client =
        somm_proto::pubsub::query_client::QueryClient::connect(grpc_endpoint.to_string()).await?;
    let request = somm_proto::pubsub::QuerySubscribersRequest {};
    let response = client.query_subscribers(request).await?;

    debug!("subscribers response: {:?}", response);

    Ok(response.into_inner().subscribers)
}

pub(crate) async fn refresh_subscriber_cache(app_handle: tauri::AppHandle) -> Result<()> {
    debug!("refreshing subscriber cache");
    let subscribers = get_subscribers(app_handle.clone()).await?;
    let app_context = app_handle.state::<Context>();

    app_context.0.write().await.subscribers = Some(subscribers);
    debug!("subscriber cache updated");

    Ok(())
}

pub(crate) async fn refresh_subscriber_cache_thread(app_handle: tauri::AppHandle) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(3000));

    // Consume the first tick if the cache is already populated, which it should be
    // if the app startup is doing its job.
    if app_handle
        .state::<Context>()
        .0
        .read()
        .await
        .subscribers
        .is_some()
    {
        interval.tick().await;
    }

    loop {
        interval.tick().await;
        if let Err(err) = refresh_subscriber_cache(app_handle.clone()).await {
            log::error!("failed to refresh subscriber cache: {err}");
        }
    }
}

pub(crate) async fn get_channel(
    publisher_identity: Identity,
    subscriber_ca: String,
    subscriber_push_url: String,
) -> Result<Channel> {
    if subscriber_push_url.contains("//") {
        bail!("subscriber push URL should not include a scheme");
    }

    let Some(port_index) = subscriber_push_url.find(':') else {
        bail!("subscriber push URL must include a port: {subscriber_push_url}");
    };
    let subscriber_domain = subscriber_push_url[..port_index].to_string();
    let tls = ClientTlsConfig::new()
        .domain_name(subscriber_domain)
        .ca_certificate(Certificate::from_pem(subscriber_ca))
        .identity(publisher_identity);

    let subscriber_push_url = format!("https://{subscriber_push_url}");
    Channel::from_shared(subscriber_push_url)?
        .tls_config(tls)?
        .connect()
        .await
        .map_err(Into::into)
}
