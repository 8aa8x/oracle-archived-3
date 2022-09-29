use std::collections::HashMap;
use std::sync::Arc;

use config::{Config as ConfigCrate, ConfigError};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ChainConfig {
    pub id: u32,
    pub rpcs: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EthereumConfig {
    pub chains: HashMap<String, ChainConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ListenersConfig {
    pub ethereum: Option<Arc<EthereumConfig>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub database_urls: Vec<String>,
    pub cache_urls: Vec<String>,
    pub listeners: Arc<ListenersConfig>,
}

/// The configuration object, usually serialized from `config.json5` unless
/// otherwise configured.
impl Config {
    /// Load the configuration file and try to bind it to the Config type. The
    /// file can be in any format supported by the `config` crate as long as the
    /// types conform to the Config struct. Recommended file type would be json5
    /// format and named `config.json5`.
    pub fn load() -> Result<Self, ConfigError> {
        ConfigCrate::builder()
            .add_source(config::File::with_name("config"))
            .build()?
            .try_deserialize::<Config>()
    }
}
