pub mod config;
mod sign;
mod balance;

use std::time::{SystemTime, UNIX_EPOCH};
use reqwest::{Client as HttpClient, Request, RequestBuilder};
use reqwest::header::HeaderMap;
use serde::Serialize;
use self::config::{RestBncCfg, BncAuthCfg};

pub struct RestClient<'a> {
    cfg: &'a RestBncCfg,
    client: HttpClient,
}

impl<'a> RestClient<'a> {
    fn auth_headers(cfg: &BncAuthCfg) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("X-MBX-APIKEY", cfg.key.parse().unwrap());
        headers
    }

    pub fn new(cfg: &'a RestBncCfg) -> Self {
        Self {
            cfg,
            client: HttpClient::builder().default_headers(Self::auth_headers(&cfg.auth)).build().unwrap(),
        }
    }

    /// Constructs full rest path from the given relative url.
    fn full_path(&self, relative: &str) -> String {
        format!("{}{}", self.cfg.baseurl, relative)
    }
}

#[cfg(test)]
mod test_utils {
    use crate::config::AppCfg;
    use crate::logging::tests::setup_test_logger;
    use super::*;

    pub struct RestTestCtx {
        cfg: AppCfg,
    }

    impl RestTestCtx {
        pub fn setup() -> Self {
            setup_test_logger();
            Self {
                cfg: AppCfg::load_testnet().unwrap()
            }
        }

        pub fn client(&self) -> RestClient {
            RestClient::new(&self.cfg.bnc.rest)
        }
    }
}