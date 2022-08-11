use serde::Deserialize;
use derive_getters::Getters;

#[derive(Debug, Clone, Getters, Deserialize)]
pub struct WsBncCfg {
    pub baseurl: String,
}

impl Default for WsBncCfg {
    fn default() -> Self {
        Self {
            baseurl: String::from("wss://stream.binance.com:9443")
        }
    }
}
