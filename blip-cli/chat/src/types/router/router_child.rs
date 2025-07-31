use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RouterChild {
    #[serde(rename = "identity")]
    pub identity: String,

    #[serde(rename = "isDefault")]
    pub is_default: bool,

    #[serde(rename = "longName")]
    pub long_name: String,

    #[serde(rename = "service")]
    pub service: String,

    #[serde(rename = "shortName")]
    pub short_name: String,
}