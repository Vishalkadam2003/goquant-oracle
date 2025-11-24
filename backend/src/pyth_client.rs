use anyhow::Result;

pub struct PythClient;

impl PythClient {
    pub fn new(_url: &str) -> Self {
        Self
    }

    pub async fn get_price_for_symbol(&self, _symbol: &str) -> Result<f64> {
        Ok(50000.0)
    }
}
