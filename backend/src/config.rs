use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub redis_url: String,
    pub postgres_url: String,
    pub solana_rpc_url: String,
    pub symbols: Vec<String>,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            redis_url: "redis://127.0.0.1/".to_string(),
            postgres_url: "postgres://postgres:Vishal123%40@localhost/oracle".to_string(),
            solana_rpc_url: "mock".to_string(),   // <── YES, "mock", NOT ANY URL
            symbols: vec!["BTC".to_string(), "ETH".to_string()],
        }
    }
}
