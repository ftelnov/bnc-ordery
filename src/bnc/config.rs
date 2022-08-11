use derive_getters::Getters;
use serde::Deserialize;
use super::ws::config::WsBncCfg;
use super::rest::config::RestBncCfg;

#[derive(Debug, Clone, Getters, Deserialize)]
pub struct BncCfg {
    #[serde(default)]
    pub ws: WsBncCfg,

    pub rest: RestBncCfg,
}
