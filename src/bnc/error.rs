use reqwest::Error;
use thiserror::Error;

/// Errors that BNC operations part can return.
#[derive(Error, Debug)]
pub enum BncError {
    #[error("Reqwest crate could not proceed with given data. Origin error: {}", .0)]
    RequestError(reqwest::Error),

    #[error("Json ops framework was unable to process entity. Possibly some binance entity is malformed. Origin error: {}", .0)]
    JsonError(serde_json::Error),

    #[error("Urlencode ops framework was unable to process entity. Origin error: {}", .0)]
    UrlencodeError(serde_urlencoded::ser::Error),

    #[error("Interaction with WS module failed. Origin error: {}", .0)]
    WsError(tokio_tungstenite::tungstenite::Error),

    #[error("Could not send thread's data to the thread's master.")]
    DataTransmitError,

    #[error("Data was rejected by predicate. Possibly some conditions were unmet.")]
    DataRejected,

    #[error("Request signing error. Possibly some entities were malformed, or query size is unsupported. Source: {}", .0)]
    SigningError(String),
}

pub type BncResult<T> = Result<T, BncError>;

impl From<reqwest::Error> for BncError {
    fn from(err: Error) -> Self {
        Self::RequestError(err)
    }
}

impl From<serde_json::Error> for BncError {
    fn from(err: serde_json::Error) -> Self {
        Self::JsonError(err)
    }
}

impl From<tokio_tungstenite::tungstenite::Error> for BncError {
    fn from(err: tokio_tungstenite::tungstenite::Error) -> Self {
        Self::WsError(err)
    }
}

impl From<serde_urlencoded::ser::Error> for BncError {
    fn from(err: serde_urlencoded::ser::Error) -> Self {
        Self::UrlencodeError(err)
    }
}