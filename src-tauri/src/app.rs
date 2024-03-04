use std::{fs, path::Path, sync::Arc};

use eyre::{bail, Result};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use somm_proto::pubsub::Subscriber;
use tokio::sync::RwLock;
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Identity};
use tracing::debug;

const DEFAULT_GRPC_ENDPOINT: &str = "https://sommelier-grpc.polkachu.com:14190";

lazy_static! {
    static ref APP_CONTEXT: Arc<RwLock<AppContext>> = Arc::new(RwLock::new(AppContext::default()));
}

// TODO: front end needs to ask the user to supply domain and cert data
#[derive(Default, Deserialize, Serialize)]
struct AppConfig {
    grpc_endpoint: Option<String>,
    publisher_domain: Option<String>,
    client_cert_path: Option<String>,
    client_cert_key_path: Option<String>,
}

fn load_config<P: AsRef<Path>>(config_path: P) -> Result<AppConfig> {
    let config = fs::read_to_string(config_path)?;

    toml::de::from_str::<AppConfig>(&config).map_err(Into::into)
}

#[derive(Default)]
pub(crate) struct AppContext {
    pub grpc_endpoint: String,
    pub publisher_domain: Option<String>,
    pub client_identity: Option<Identity>,
    // TODO: cache subscribers on disk
    pub subscribers: Option<Vec<Subscriber>>,
}

fn initialize_app_context(config: AppConfig) -> Result<()> {
    let mut app_context = futures::executor::block_on(APP_CONTEXT.write());
    let grpc_endpoint = config
        .grpc_endpoint
        .unwrap_or(DEFAULT_GRPC_ENDPOINT.to_string());
    let client_identity =
        if config.client_cert_path.is_some() && config.client_cert_key_path.is_some() {
            Some(get_publisher_identity(
                config.client_cert_path.unwrap(),
                config.client_cert_key_path.unwrap(),
            )?)
        } else {
            None
        };

    *app_context = AppContext {
        grpc_endpoint,
        publisher_domain: config.publisher_domain,
        client_identity,
        subscribers: None,
    };

    Ok(())
}

pub(crate) async fn get_app_context() -> tokio::sync::RwLockReadGuard<'static, AppContext> {
    APP_CONTEXT.read().await
}

fn get_publisher_identity(cert_path: String, cert_key_path: String) -> Result<Identity> {
    let client_cert = fs::read_to_string(cert_path)?;
    let client_key = fs::read_to_string(cert_key_path)?;

    Ok(Identity::from_pem(client_cert, client_key))
}

async fn get_subscribers(grpc_endpoint: &str) -> Result<Vec<Subscriber>> {
    let mut client =
        somm_proto::pubsub::query_client::QueryClient::connect(grpc_endpoint.to_string()).await?;
    let request = somm_proto::pubsub::QuerySubscribersRequest {};
    let response = client.query_subscribers(request).await?;

    debug!("subscribers response: {:?}", response);

    Ok(response.into_inner().subscribers)
}

async fn refresh_subscriber_cache(grpc_endpoint: &str) -> Result<()> {
    debug!("refreshing subscriber cache");
    let subscribers = get_subscribers(grpc_endpoint).await?;
    let mut app_context = APP_CONTEXT.write().await;

    app_context.subscribers = Some(subscribers);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_subscribers_cache() {
        refresh_subscriber_cache(DEFAULT_GRPC_ENDPOINT)
            .await
            .unwrap();

        let subscribers = &get_app_context().await.subscribers;
        assert!(subscribers.is_some());
        assert!(subscribers.to_owned().unwrap().len() > 0);
    }
}
