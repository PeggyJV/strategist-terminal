use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub(crate) const CONFIG_FILE_ENV_VAR: &str = "ST_CONFIG_FILE";
pub(crate) const DEFAULT_CONFIG_FILE: &str = "./config.toml";
pub(crate) const DEFAULT_RPC_ENDPOINT: &str = "https://sommelier-rpc.polkachu.com:443";
pub(crate) const DEFAULT_GRPC_ENDPOINT: &str = "https://sommelier-grpc.polkachu.com:14190";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppConfig {
    pub rpc_endpoint: Option<String>,
    pub grpc_endpoint: Option<String>,
    pub publisher_domain: Option<String>,
    pub client_cert_path: Option<String>,
    pub client_cert_key_path: Option<String>,
}

impl AppConfig {
    pub fn load() -> Self {
        let config_path = match std::env::var(CONFIG_FILE_ENV_VAR) {
            Ok(path) => Path::new(&path).to_path_buf(),
            Err(_) => Path::new(DEFAULT_CONFIG_FILE).to_path_buf(),
        };

        if config_path.exists() {
            let config_str = fs::read_to_string(&config_path).expect("Failed to read config file");
            toml::from_str(&config_str).expect("Failed to parse config file")
        } else {
            Self::default()
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = match std::env::var(CONFIG_FILE_ENV_VAR) {
            Ok(path) => Path::new(&path).to_path_buf(),
            Err(_) => Path::new(DEFAULT_CONFIG_FILE).to_path_buf(),
        };

        let config_str = toml::to_string(self)?;
        fs::write(config_path, config_str)?;
        Ok(())
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            rpc_endpoint: Some(DEFAULT_RPC_ENDPOINT.to_string()),
            grpc_endpoint: Some(DEFAULT_GRPC_ENDPOINT.to_string()),
            publisher_domain: None,
            client_cert_path: None,
            client_cert_key_path: None,
        }
    }
}

impl std::fmt::Display for AppConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AppConfig {{ rpc_endpoint: {:?}, grpc_endpoint: {:?}, publisher_domain: {:?}, client_cert_path: {:?}, client_cert_key_path: {:?} }}",
               self.rpc_endpoint, self.grpc_endpoint, self.publisher_domain, self.client_cert_path, self.client_cert_key_path)
    }
}

impl log::kv::ToValue for AppConfig {
    fn to_value(&self) -> log::kv::Value {
        log::kv::Value::from_serde(self)
    }
}
