use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DynamicContent {
    #[serde(rename = "type")]
    pub dynamic_message_type: String,

    #[serde(rename = "rawContent")]
    pub raw_content: String,

    #[serde(rename = "metadata")]
    pub metadata: Option<HashMap<String, String>>,
}