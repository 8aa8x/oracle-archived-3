use std::sync::atomic::Ordering;
use std::sync::Arc;

use async_trait::async_trait;
use futures::future::{BoxFuture, Future};
use slog::{debug, error, info, o, Logger};
use thiserror::Error;
use tokio::task::JoinSet;
use tokio::time::{sleep, timeout, Duration};
use web3::transports::{Either, Http, WebSocket};
use web3::Web3;

use crate::config::{ChainConfig, EthereumConfig};
use crate::listeners::BaseListener;
use crate::oracle::KillSwitchAware;

type Transport = Either<WebSocket, Http>;

#[derive(Error, Debug)]
pub enum EthereumListenerError {
    #[error("no valid clients could be made from the rpcs in the config")]
    NoValidClients,
}

#[derive(Debug)]
struct EthereumListener {
    /// The base listener
    base: Arc<BaseListener>,
    /// The web3 client used to interact with RPCs
    client: Web3<Transport>,
    /// The Ethereum listener logger
    logger: Logger,
}

type PinFutureTask<Input, Output> = fn(Arc<Input>) -> BoxFuture<'static, Output>;

#[async_trait]
impl KillSwitchAware for EthereumListener {
    async fn run_with_kill_switch_awareness<F, Fut>(self: Arc<Self>, f: F, refresh: Duration)
    where
        F: Fn(Arc<Self>) -> Fut + Send + Sync,
        Fut: Future<Output = ()> + Send,
    {
        loop {
            // Check kill switch before running again
            if self.clone().base.oracle.kill_switch.load(Ordering::Relaxed) {
                break;
            }

            // Call the wrapped function with a timeout
            if let Err(e) = timeout(refresh, f(self.clone())).await {
                error!(self.logger, "Function timed out: {e}");
            }

            // Fetch block_confirmations
            sleep(refresh).await;
        }
    }
}

/// Single Ethereum listener that can execute on the client
impl EthereumListener {
    /// Create an Ethereum listener
    ///
    /// Automatically uses the config in the BaseListener to generate a client
    /// with provided RPCs.
    pub async fn create_from_chain_config(
        base: Arc<BaseListener>,
        name: String,
        config: ChainConfig,
        logger: Logger,
    ) -> Result<Self, EthereumListenerError> {
        let logger = logger.new(o!("ethereum" => name.clone()));

        for rpc in &config.rpcs {
            let transport = match rpc {
                rpc if rpc.starts_with("http") => match Http::new(rpc) {
                    Ok(rpc) => Some(Transport::Right(rpc)),
                    Err(e) => {
                        error!(logger, "Error creating HTTP client ({rpc})");
                        debug!(logger, "{e:?}");
                        None
                    }
                },
                rpc if rpc.starts_with("ws") => match WebSocket::new(rpc).await {
                    Ok(rpc) => Some(Transport::Left(rpc)),
                    Err(e) => {
                        error!(logger, "Error creating WebSocket client ({rpc})", rpc = rpc);
                        debug!(logger, "{e:?}");
                        None
                    }
                },
                _ => {
                    error!(
                        logger,
                        "RPC ({rpc}) must be either a WebSockets or HTTP URL",
                        rpc = rpc,
                    );
                    None
                }
            };

            match transport {
                Some(transport) => {
                    return Ok(EthereumListener {
                        base,
                        client: web3::Web3::new(transport),
                        logger,
                    });
                }
                None => {
                    continue;
                }
            };
        }

        Err(EthereumListenerError::NoValidClients)
    }

    /// Start the main tasks for the listener
    pub async fn listen(self: Arc<Self>) {
        let mut join_handles = JoinSet::new();
        // A list of tasks that all have the same function signature
        let tasks: Vec<PinFutureTask<EthereumListener, ()>> = vec![
            |listener| Box::pin(EthereumListener::gather_block_confirmations(listener)),
            |listener| Box::pin(EthereumListener::check_retries_for_new_client(listener)),
        ];

        // Run tasks in parallel
        for task in tasks {
            join_handles.spawn({
                let listener = self.clone();
                async move {
                    task(listener).await;
                }
            });
        }

        // Wait for all handles to finish before returning
        while let Some(res) = join_handles.join_next().await {
            res.unwrap();
        }
    }

    /// Periodically fetches all block confirmation levels from the database.
    /// The block confirmation levels are a unique list of all the
    /// `confirmations` column on the `listeners.ethereum_events` table.
    async fn gather_block_confirmations(self: Arc<Self>) {
        self.run_with_kill_switch_awareness(
            |listener| {
                info!(listener.logger, "Gathering new block confirmations");
                sleep(Duration::from_secs(3))
            },
            Duration::from_secs(3),
        )
        .await;
    }

    /// Periodically checks to see if the retries are too high, and if so it
    /// will switch the client.
    async fn check_retries_for_new_client(self: Arc<Self>) {
        self.run_with_kill_switch_awareness(
            |listener| {
                info!(listener.logger, "Checking retries for new client");
                sleep(Duration::from_secs(3))
            },
            Duration::from_secs(3),
        )
        .await;
    }
}

#[derive(Debug)]
pub struct EthereumListeners {
    /// The base listener
    base: Arc<BaseListener>,
    /// The ethereum portion of configuration
    config: Arc<EthereumConfig>,
    /// The logger for all Ethereum listeners
    logger: Logger,
}

/// Multiple Ethereum listeners for all configurations provided
impl EthereumListeners {
    /// Instantiate a new Ethereum Listener
    pub fn new(base: Arc<BaseListener>) -> Self {
        Self {
            base: base.clone(),
            config: base.config.ethereum.clone().unwrap(),
            logger: base.logger.new(o!("listener" => "ethereum")),
        }
    }

    /// Starts the Ethereum Listener
    pub async fn start_ethereum_listeners(self) {
        // Create a list of listener handlers
        let mut join_handles = JoinSet::new();

        for listener in self.create_listeners().await {
            join_handles.spawn({
                let arc = Arc::new(listener);
                async move {
                    EthereumListener::listen(arc).await;
                }
            });
        }

        // Wait for all handles to finish before returning
        while let Some(res) = join_handles.join_next().await {
            res.unwrap();
        }
    }

    /// Creates an Ethereum listener for every chain configured
    async fn create_listeners(&self) -> Vec<EthereumListener> {
        let mut listeners = vec![];

        for (name, config) in &self.config.chains {
            info!(
                self.logger,
                "Found configuration for chain {name} ({})", config.id
            );

            match EthereumListener::create_from_chain_config(
                self.base.clone(),
                name.clone(),
                config.clone(),
                self.logger.clone(),
            )
            .await
            {
                Ok(listener) => listeners.push(listener),
                Err(error) => {
                    error!(self.logger, "Could not create a listener for {name}");
                    debug!(self.logger, "{error}");
                }
            }
        }

        listeners
    }
}
