use eyre::{bail, Result};

#[derive(Debug, Clone)]
pub(crate) enum Rpc {
    Arbitrum,
    Ethereum,
}

impl std::fmt::Display for Rpc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rpc::Arbitrum => write!(f, "https://arbitrum.llamarpc.com"),
            Rpc::Ethereum => write!(f, "https://eth.llamarpc.com"),
        }
    }
}

impl Rpc {
    pub(crate) fn from_chain_id(chain_id: u64) -> Result<Self> {
        Ok(match chain_id {
            1 => Rpc::Ethereum,
            42161 => Rpc::Arbitrum,
            _ => bail!("Unsupported chain id: {}", chain_id),
        })
    }
}
