use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QuickReplyOption {
    #[serde(rename = "text")]
    pub text: String,

    #[serde(rename = "previewText")]
    pub preview_text: String,

    #[serde(rename = "value")]
    pub value: Option<String>,

    #[serde(rename = "index")]
    pub index: u32,

    #[serde(rename = "type")]
    pub option_type: Option<String>
}