use anyhow::Result;

pub struct SwitchboardClient;

impl SwitchboardClient {
    pub fn new(_url: &str) -> Self {
        Self
    }

    pub async fn get_price_for_symbol(&self, _symbol: &str) -> Result<f64> {
        Ok(49950.0)
    }
}
