use serde::Deserialize;
use derive_getters::Getters;

#[derive(Debug, Clone, Getters, Deserialize)]
pub struct BncAuthCfg {
    pub key: String,
    pub secret: String,
}

#[derive(Debug, Clone, Getters, Deserialize)]
pub struct RestBncCfg {
    #[serde(default = "default_rest_baseurl")]
    pub baseurl: String,
    pub auth: BncAuthCfg,
}

fn default_rest_baseurl() -> String {
    String::from("https://api.binance.com")
}
