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

pub(crate) async fn apply_config(app_handle: tauri::AppHandle, config: AppConfig) -> Result<()> {
    let app_context = app_handle.state::<Context>();
    let client_identity = match (config.client_cert_path, config.client_cert_key_path) {
        (Some(cert_path), Some(key_path)) => Some(get_publisher_identity(cert_path, key_path)?),
        _ => None,
    };

    *app_context.0.write().await = AppContext {
        rpc_endpoint: config
            .rpc_endpoint
            .unwrap_or(DEFAULT_RPC_ENDPOINT.to_string()),
        grpc_endpoint: config
            .grpc_endpoint
            .unwrap_or(DEFAULT_GRPC_ENDPOINT.to_string()),
        publisher_domain: config.publisher_domain,
        client_identity,
        subscribers: None,
    };

    Ok(())
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

    Ok(())
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
