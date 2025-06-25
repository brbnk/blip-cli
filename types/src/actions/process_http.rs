use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessHttp {
    #[serde(rename = "headers")]
    pub headers: HashMap<String, String>,

    #[serde(rename = "method")]
    pub method: String,

    #[serde(rename = "uri")]
    pub uri: String,

    #[serde(rename = "responseStatusVariable")]
    pub status: String,

    #[serde(rename = "responseBodyVariable")]
    pub response: String
}