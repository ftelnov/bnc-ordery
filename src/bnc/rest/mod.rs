pub mod config;
mod sign;
mod balance;

use std::time::{SystemTime, UNIX_EPOCH};
use reqwest::{Client as HttpClient, Request, RequestBuilder};
use reqwest::header::HeaderMap;
use serde::Serialize;
use self::config::{RestBncCfg, BncAuthCfg};

struct RestClient<'a> {
    cfg: &'a RestBncCfg,
    client: HttpClient,
}

impl<'a> RestClient<'a> {
    fn auth_headers(cfg: &BncAuthCfg) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("X-MBX-APIKEY", cfg.key.parse().unwrap());
        headers
    }

    fn new(cfg: &'a RestBncCfg) -> Self {
        Self {
            cfg,
            client: HttpClient::builder().default_headers(Self::auth_headers(&cfg.auth)).build().unwrap(),
        }
    }
}
