use std::fmt::{Display, Formatter};
use serde::{de, Deserialize};

fn deserialize_amount<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;
    s.parse()
        .map_err(|_| de::Error::custom("Invalid amount format"))
}

/// Encapsulate string provided by binance and store it as f64.
#[derive(Deserialize, Clone, Debug, PartialOrd, PartialEq)]
pub struct Amount(#[serde(deserialize_with = "deserialize_amount")] f64);

impl Display for Amount {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

impl Default for Amount {
    fn default() -> Self {
        Self(0.0)
    }
}
