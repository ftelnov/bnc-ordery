use chrono::{DateTime, Utc};
use crate::bnc::data::Amount;
use crate::bnc::error::BncResult;
use serde::Deserialize;
use chrono::serde::ts_seconds;
use crate::bnc::rest::sign::RequestSign;
use super::RestClient;
use async_trait::async_trait;
use log::info;

#[derive(Deserialize, Clone, Debug)]
struct AssetBalance {
    pub asset: String,
    pub free: Amount,
    pub locked: Amount,
}

#[derive(Deserialize, Clone, Debug)]
struct BalancesInfo {
    pub balances: Vec<AssetBalance>,

    #[serde(with = "ts_seconds", rename = "updateTime")]
    pub update_time: DateTime<Utc>,
}

#[async_trait]
pub trait BalanceFetch {
    /// Fetches current state of account's balances.
    ///
    /// It's Weight(IP) is equal to 10.
    async fn fetch_account_balances(&self) -> BncResult<BalancesInfo>;
}

#[async_trait]
impl<'a> BalanceFetch for RestClient<'a> {
    async fn fetch_account_balances(&self) -> BncResult<BalancesInfo> {
        let builder = self.client.get(self.full_path("/api/v3/account"));
        let request = self.sign_request(builder, &())?;


        let result = self.client.execute(request).await?;

        info!("Received headers are: {:?}", result.headers());
        let response: BalancesInfo = result.json().await?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use super::super::test_utils::RestTestCtx;
    use crate::config::AppCfg;

    #[tokio::test]
    async fn it_fetches_test_account_balances() -> Result<()> {
        let ctx = RestTestCtx::setup();
        let current = Utc::now();

        let balances = ctx.client().fetch_account_balances().await?;

        assert!(current < balances.update_time);

        Ok(())
    }
}
