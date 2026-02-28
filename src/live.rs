use serde::Serialize;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::broadcast;

use crate::cache::AppCache;
use crate::rpc::DashRpcClient;

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum LiveEvent {
    NewBlock {
        hash: String,
        height: u64,
        time: u64,
        n_tx: u32,
        chainlock: bool,
        credit_pool_balance: f64,
    },
    MempoolUpdate {
        size: u64,
        bytes: u64,
        total_fee: f64,
    },
}

pub struct LiveUpdater {
    rpc: Arc<DashRpcClient>,
    cache: Arc<AppCache>,
    tx: broadcast::Sender<LiveEvent>,
    last_height: AtomicU64,
    last_mempool_size: AtomicU64,
}

impl LiveUpdater {
    pub fn new(
        rpc: Arc<DashRpcClient>,
        cache: Arc<AppCache>,
        tx: broadcast::Sender<LiveEvent>,
    ) -> Self {
        Self {
            rpc,
            cache,
            tx,
            last_height: AtomicU64::new(0),
            last_mempool_size: AtomicU64::new(0),
        }
    }

    pub async fn run(self) {
        tracing::info!("Live updater started");

        let mut tick = 0u64;

        loop {
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            tick += 1;

            // Check for new blocks every 2s
            if let Ok(height) = self.rpc.get_block_count().await {
                let prev = self.last_height.load(Ordering::Relaxed);
                if prev > 0 && height > prev {
                    // New block(s) detected
                    for h in (prev + 1)..=height {
                        if let Ok(hash) = self.rpc.get_block_hash(h).await {
                            if let Ok(block) = self.rpc.get_block(&hash, 1).await {
                                let credit_pool = block
                                    .cb_tx
                                    .as_ref()
                                    .map(|cb| cb.credit_pool_balance)
                                    .unwrap_or(0.0);

                                let event = LiveEvent::NewBlock {
                                    hash: block.hash.clone(),
                                    height: block.height,
                                    time: block.time,
                                    n_tx: block.n_tx,
                                    chainlock: block.chainlock,
                                    credit_pool_balance: credit_pool,
                                };

                                let _ = self.tx.send(event);
                                tracing::info!("New block {} at height {}", &hash[..16], h);
                            }
                        }
                    }
                    // Invalidate tip caches
                    self.cache.latest_blocks.invalidate_all();
                    self.cache.status.invalidate_all();
                }
                self.last_height.store(height, Ordering::Relaxed);
            }

            // Check mempool every 5 ticks (10s)
            if tick % 5 == 0 {
                if let Ok(mempool) = self.rpc.get_mempool_info().await {
                    let prev_size = self.last_mempool_size.load(Ordering::Relaxed);
                    if mempool.size != prev_size {
                        let event = LiveEvent::MempoolUpdate {
                            size: mempool.size,
                            bytes: mempool.bytes,
                            total_fee: mempool.total_fee,
                        };
                        let _ = self.tx.send(event);
                        self.last_mempool_size.store(mempool.size, Ordering::Relaxed);
                    }
                }
            }
        }
    }
}
