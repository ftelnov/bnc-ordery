use config::{Config, ConfigError, Environment, File};
use derive_getters::Getters;
use serde::Deserialize;
use std::env;
use crate::bnc::config::BncCfg;
use crate::logging::LogCfg;
// use crate::ui::config::UICfg;

const BASE_CONFIG_DIR: &str = "config";

#[derive(Getters, Debug, Clone, Deserialize)]
pub struct AppCfg {
    /// Logging configuration part of the application.
    #[serde(default)]
    pub logging: LogCfg,

    pub bnc: BncCfg,
}

impl AppCfg {
    /// Load current app's configuration in the following order:
    ///
    /// 1) From default.* files;
    /// 2) From the environment file(environment is set via RUN_MODE env variable).
    ///     e.g. If you have RUN_MODE=dev, it will load dev.*;
    /// 3) From local.* files;
    /// 4) Finally, from environment variables prefixed with standard prefix.
    ///
    pub fn load() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "dev".into());
        let default_config_file = format!("{}/default", BASE_CONFIG_DIR);
        let environment_file = format!("{}/{}", BASE_CONFIG_DIR, run_mode);
        let local_file = format!("{}/local", BASE_CONFIG_DIR);

        let s = Config::builder()
            .add_source(File::with_name(&default_config_file).required(false))
            .add_source(File::with_name(&environment_file).required(false))
            .add_source(File::with_name(&local_file).required(false))
            .add_source(Environment::with_prefix("app").separator("_"))
            .build()?;

        s.try_deserialize()
    }
}

#[cfg(test)]
pub mod test_utils {
    use super::*;

    impl AppCfg {
        /// Load config as default one, then alters its endpoints so they are point to binance testnet.
        pub fn load_testnet() -> Result<Self, ConfigError> {
            let mut cfg = Self::load()?;
            cfg.bnc.rest.baseurl = String::from("https://testnet.binance.vision");
            cfg.bnc.ws.baseurl = String::from("wss://stream.binance.com:9443");
            Ok(cfg)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ensures that global application's data is properly formatted.
    // If you see this test as failed one, check whether current config sources are OK for you.
    // Mostly failure means that you missed some required parameter.
    #[test]
    fn it_deserializes_default_app_config() {
        AppCfg::load().unwrap();
    }
}
