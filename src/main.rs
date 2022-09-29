mod config;
mod listeners;
mod oracle;
mod utils;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use anyhow::Result;
use lazy_static::lazy_static;
use nanoid::nanoid;
use signal_hook::consts::TERM_SIGNALS;
use signal_hook::iterator::Signals;
use slog::{warn, Logger};
use tokio::task::JoinSet;

use crate::config::Config;
use crate::listeners::BaseListener;
use crate::oracle::Oracle;
use crate::utils::log::create_logger;
use crate::utils::{postgres, redis};

// Initialize logger
lazy_static! {
    static ref BASE_LOGGER: Logger = create_logger(&nanoid!());
}

/// The main kill switch loop constantly listens for any terminate signals
async fn start_kill_switch_loop(kill_switch: Arc<AtomicBool>) {
    let mut term_signals = Signals::new(TERM_SIGNALS).unwrap();

    if term_signals.wait().next().is_some() {
        warn!(
            BASE_LOGGER,
            "Kill signal received -- attempting to clean up"
        );
        kill_switch.store(true, Ordering::Relaxed);
    };
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration file
    let config = Arc::new(Config::load()?);

    let oracle = Arc::new(Oracle {
        config: config.clone(),
        kill_switch: Arc::new(AtomicBool::new(false)),
        postgres_client: Arc::new(
            postgres::create_client(config.database_urls.first().unwrap()).await?,
        ),
        redis_client: Arc::new(redis::create_lock_client(config.cache_urls.clone())?),
        logger: BASE_LOGGER.clone(),
    });

    // Thread handles that will await until all are finished
    let mut join_handles = JoinSet::new();

    // All threads should listen for this kill switch and handle it properly
    join_handles.spawn(start_kill_switch_loop(oracle.kill_switch.clone()));

    // Start the listeners thread
    join_handles.spawn(BaseListener::new(oracle.clone()).start_listeners());

    // Start the adapters thread

    // Start the responders thread

    // Wait for all threads to finish before returning
    while let Some(res) = join_handles.join_next().await {
        res?;
    }

    Ok(())
}

// use hex_literal::hex;
// use std::time;
// use web3::{futures, futures::StreamExt, types::FilterBuilder};

// #[tokio::main]
// async fn main() -> web3::contract::Result<()> {
//     let _ = env_logger::try_init();
//     let web3 = web3::Web3::new(
//         web3::transports::WebSocket::new(
//             "wss://mainnet.infura.io/ws/v3/0b008b83f1eb4173b5178b47b644a175",
//         )
//         .await?,
//     );

//     // Filter for Hello event in our contract
//     let filter = FilterBuilder::default()
//         .address(vec![hex!("57f1887a8BF19b14fC0dF6Fd9B2acc9Af147eA85").into()])
//         .topics(
//             Some(vec![hex!(
//                 "ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
//             )
//             .into()]),
//             None,
//             None,
//             None,
//         )
//         .build();

//     let filter = web3.eth_filter().create_logs_filter(filter).await?;

//     let logs_stream = filter.stream(time::Duration::from_secs(1));
//     futures::pin_mut!(logs_stream);

//     loop {
//         let log = logs_stream.next().await.unwrap();
//         println!("got log: {:#?}", log.unwrap());
//     }
// }
