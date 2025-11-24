use deadpool_postgres::{Manager, Pool};
use tokio_postgres::NoTls;
use anyhow::Result;

use crate::models::ConsensusPrice;

#[derive(Clone)]
pub struct Database {
    pub pool: Pool,
}

impl Database {
    pub fn new(conn_str: &str) -> Self {
        let mgr = Manager::new(conn_str.parse().unwrap(), NoTls);

        let pool = Pool::builder(mgr)
            .max_size(16)
            .build()
            .unwrap();

        Self { pool }
    }

    pub async fn insert_price(&self, price: &ConsensusPrice) -> Result<()> {
        let client = self.pool.get().await?;

        // Format f64 as a numeric literal for PostgreSQL
        let price_str = format!("{:.12}", price.price);

        // We inline the numeric literal directly into the SQL.
        // This avoids the driver trying to map Rust String → NUMERIC param type.
        let query = format!(
            "INSERT INTO price_history (symbol, price, timestamp, sources_used)
             VALUES ($1, {}, $2, $3)",
            price_str
        );

        client
            .execute(
                &query,
                &[
                    &price.symbol,
                    &price.timestamp,
                    &(price.sources_used as i32),
                ],
            )
            .await?;

        Ok(())
    }
    pub async fn fetch_history(&self, symbol: &str) -> Result<Vec<ConsensusPrice>> {
    let client = self.pool.get().await?;

    let stmt = "
        SELECT symbol, price, timestamp, sources_used 
        FROM price_history
        WHERE symbol = $1
        ORDER BY timestamp DESC
        LIMIT 100
    ";

    let rows = client.query(stmt, &[&symbol]).await?;

    let mut out = Vec::new();
    for row in rows {
        out.push(ConsensusPrice {
            symbol: row.get::<_, String>(0),
            price: row.get::<_, f64>(1),
            timestamp: row.get::<_, i64>(2),
            sources_used: row.get::<_, i32>(3) as usize,
        });
    }

    Ok(out)
}

}
