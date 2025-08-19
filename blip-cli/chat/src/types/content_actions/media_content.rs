use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaContent {
    #[serde(rename = "type")]
    pub media_type: String,

    #[serde(rename = "uri")]
    pub uri: String,

    #[serde(rename = "text")]
    pub text: Option<String>,
}