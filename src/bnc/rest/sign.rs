use std::time::{SystemTime, UNIX_EPOCH};
use reqwest::{Request, RequestBuilder};
use serde::Serialize;
use super::RestClient;
use hmac::{Hmac as BasicHmac, Mac};
use sha2::Sha256;
use crate::bnc::error::BncError::SigningError;
use crate::bnc::error::BncResult;

type Hmac = BasicHmac<Sha256>;

/// Get "seconds" since unix epoch.
///
/// Seconds are to be multiplied by 1000 for using in BNC api.
fn get_timestamp() -> u64 {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).expect("Your clock is behind UNIX_EPOCH!");
    duration.as_secs() * 1000
}

/// Encapsulates timestamp adding to an existing query.
#[derive(Serialize)]
struct TimestampedQuery<'a, T: Serialize> {
    timestamp: u64,

    #[serde(flatten)]
    query: &'a T,
}

impl<'a, T: Serialize> TimestampedQuery<'a, T> {
    fn new(query: &'a T) -> Self {
        Self {
            timestamp: get_timestamp(),
            query,
        }
    }
}

/// Encapsulates signature creation for the provided query.
#[derive(Serialize)]
struct SignedQuery<'a, T: Serialize> {
    signature: String,

    #[serde(flatten)]
    query: &'a T,
}

impl<'a, T: Serialize> SignedQuery<'a, T> {
    /// Generates signature for the provided query and secret_key.
    fn new(query: &'a T, secret_key: &str) -> BncResult<Self> {
        let mut signature = Hmac::new_from_slice(secret_key.as_bytes()).map_err(|err| SigningError(err.to_string()))?;
        signature.update(serde_urlencoded::to_string(query)?.as_bytes());
        Ok(
            Self {
                signature: hex::encode(
                    signature.finalize()
                        .into_bytes()),
                query,
            }
        )
    }
}

pub trait RequestSign {
    /// Add timestamp and signature params to the future request.
    ///
    /// After that you should not modify request, so it will return finalized version.
    fn sign_request<T: Serialize>(&self, builder: RequestBuilder, query: &T) -> BncResult<Request>;
}

impl<'a> RequestSign for RestClient<'a> {
    fn sign_request<T: Serialize>(&self, builder: RequestBuilder, query: &T) -> BncResult<Request> {
        let timestamped = TimestampedQuery::new(query);
        let signed = SignedQuery::new(&timestamped, &self.cfg.auth.secret)?;
        Ok(builder.query(&signed).build()?)
    }
}