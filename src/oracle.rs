use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use async_trait::async_trait;
use futures::future::Future;
use rslock::RedLock;
use slog::Logger;
use sqlx::{pool::Pool, postgres::Postgres};
use tokio::time::Duration;

use crate::config::Config;

/// Contains fields for the entire running process
#[derive(Debug)]
pub struct Oracle {
    /// The oracle configuration
    pub config: Arc<Config>,
    /// A kill switch that signals the listeners should stop
    pub kill_switch: Arc<AtomicBool>,
    /// A Postgres client (sqlx)
    pub postgres_client: Arc<Pool<Postgres>>,
    /// A Redis RedLock client (rslock)
    pub redis_client: Arc<RedLock>,
    /// The base logger
    pub logger: Logger,
}

#[async_trait]
pub trait KillSwitchAware {
    /// Wrapper function to run a loop with kill switch awareness.
    async fn run_with_kill_switch_awareness<F, Fut>(self: Arc<Self>, f: F, refresh: Duration)
    where
        F: Fn(Arc<Self>) -> Fut + Send + Sync,
        Fut: Future<Output = ()> + Send;
}
