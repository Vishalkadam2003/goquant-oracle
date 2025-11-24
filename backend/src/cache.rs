use redis::{AsyncCommands, Client};
use crate::models::ConsensusPrice;
use anyhow::Result;

#[derive(Clone)]
pub struct Cache {
    pub client: Client,
}

impl Cache {
    pub fn new(url: &str) -> Self {
        Self {
            client: Client::open(url).unwrap(),
        }
    }

    pub async fn set_price(&self, symbol: &str, price: &ConsensusPrice) -> Result<()> {
        let mut conn = self.client.get_async_connection().await?;

        let json = serde_json::to_string(price)?;

        // Fixed: add explicit type annotation for Redis
        conn.set::<_, _, ()>(symbol, json).await?;

        Ok(())
    }

    pub async fn get_price(&self, symbol: &str) -> Result<Option<ConsensusPrice>> {
        let mut conn = self.client.get_async_connection().await?;

        let val: Option<String> = conn.get(symbol).await?;

        if let Some(j) = val {
            Ok(Some(serde_json::from_str(&j)?))
        } else {
            Ok(None)
        }
    }
}
