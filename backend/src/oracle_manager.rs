use std::{time::Duration, collections::HashMap};
use crate::routes::ws::broadcast_price;

use anyhow::Result;
use tracing::{info, warn};

use crate::{
    pyth_client::PythClient,
    switchboard_client::SwitchboardClient,
    price_aggregator::PriceAggregator,
    models::PriceData,
    utils::now_ts,
    SharedState,
    config::AppConfig,
};

pub struct OracleManager {
    pyth: PythClient,
    switchboard: SwitchboardClient,
    state: SharedState,
    symbols: Vec<String>,

    // NEW: for assignment requirements
    last_price: HashMap<String, f64>,
    last_update: HashMap<String, i64>,
}

impl OracleManager {
    pub fn new(cfg: &AppConfig, state: SharedState) -> Self {
        let pyth = PythClient::new(&cfg.solana_rpc_url);
        let switchboard = SwitchboardClient::new(&cfg.solana_rpc_url);

        // initialize health tracking
        let mut last_price = HashMap::new();
        let mut last_update = HashMap::new();

        for s in &cfg.symbols {
            last_price.insert(s.clone(), 0.0);
            last_update.insert(s.clone(), 0);
        }

        Self {
            pyth,
            switchboard,
            state,
            symbols: cfg.symbols.clone(),
            last_price,
            last_update,
        }
    }

    pub async fn run(mut self) {
        info!("OracleManager started with symbols: {:?}", self.symbols);

        loop {
            if let Err(e) = self.tick().await {
                warn!("OracleManager tick error: {:?}", e);
            }

            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    }

    async fn tick(&mut self) -> Result<()> {
        for symbol in self.symbols.clone() {
            let mut sources = Vec::new();
            let ts = now_ts();

            // PYTH
            if let Ok(p) = self.pyth.get_price_for_symbol(&symbol).await {
                sources.push(PriceData {
                    symbol: symbol.clone(),
                    price: p,
                    confidence: 10.0,
                    timestamp: ts,
                    source: "pyth".to_string(),
                });
            }

            // SWITCHBOARD
            if let Ok(p) = self.switchboard.get_price_for_symbol(&symbol).await {
                sources.push(PriceData {
                    symbol: symbol.clone(),
                    price: p,
                    confidence: 15.0,
                    timestamp: ts,
                    source: "switchboard".to_string(),
                });
            }

            if sources.is_empty() {
                continue;
            }

            if let Some(consensus) = PriceAggregator::aggregate(&symbol, sources) {

                // ---- STORE IN REDIS + POSTGRES ----
                self.state.cache.set_price(&symbol, &consensus).await?;
                self.state.db.insert_price(&consensus).await?;

                // ---- WEBSOCKET: push consensus ----
                let json = serde_json::to_string(&consensus)?;
                broadcast_price(&json);

                // ---- NEW: CONFIDENCE CHANGE EVENT (Mock) ----
                let conf_msg = format!(
                    "{{\"type\":\"confidence\",\"symbol\":\"{}\",\"confidence\":{}}}",
                    symbol, 10
                );
                broadcast_price(&conf_msg);

                // ---- NEW: PRICE JUMP ALERT (>3%) ----
                if let Some(prev) = self.last_price.get(&symbol) {
                    if *prev != 0.0 {
                        let pct = ((consensus.price - prev) / prev).abs() * 100.0;
                        if pct >= 3.0 {
                            let alert = format!(
                                "{{\"type\":\"large_move\",\"symbol\":\"{}\",\"change_pct\":{}}}",
                                symbol, pct
                            );
                            broadcast_price(&alert);
                        }
                    }
                }
                self.last_price.insert(symbol.clone(), consensus.price);

                // ---- NEW: HEALTH / STALE ALERT (>3s no update) ----
                let now = now_ts();
                if let Some(last) = self.last_update.get(&symbol) {
                    if now - last > 3 {
                        let alert = format!(
                            "{{\"type\":\"stale\",\"symbol\":\"{}\",\"seconds\":{}}}",
                            symbol,
                            now - last
                        );
                        broadcast_price(&alert);
                    }
                }
                self.last_update.insert(symbol.clone(), now);

                // Logging
                info!(
                    "Updated consensus price for {}: {} (sources: {})",
                    symbol, consensus.price, consensus.sources_used
                );
            }
        }

        Ok(())
    }
}
