use std::sync::Arc;

use slog::{info, o, Logger};
use tokio::task::JoinSet;

use crate::config::ListenersConfig;
use crate::listeners::ethereum::EthereumListeners;
use crate::oracle::Oracle;

/// Contains metadata for an actual listener
#[derive(Debug)]
pub struct BaseListener {
    /// The base oracle
    pub oracle: Arc<Oracle>,
    /// The listeners configuration
    pub config: Arc<ListenersConfig>,
    /// The logger for listeners
    pub logger: Logger,
}

impl BaseListener {
    /// Instantiate a new `BaseListener` with required fields
    pub fn new(oracle: Arc<Oracle>) -> Self {
        BaseListener {
            logger: oracle.logger.new(o!("module" => "listener")),
            oracle: oracle.clone(),
            config: oracle.config.listeners.clone(),
        }
    }

    /// The main listener thread that spawns all specific listeners if they are configured
    pub async fn start_listeners(self) {
        // Create the BaseListener Arc to share between listener threads
        let base = Arc::new(self);

        // Create a list of listener handlers
        let mut join_handles = JoinSet::new();

        // Start the Ethereum listener thread if configured
        if base.config.ethereum.is_some() {
            info!(base.logger, "Starting the Ethereum listeners");
            join_handles.spawn(EthereumListeners::new(base.clone()).start_ethereum_listeners());
        }

        // Wait for all handles to finish before returning
        while let Some(res) = join_handles.join_next().await {
            res.unwrap();
        }
    }
}
