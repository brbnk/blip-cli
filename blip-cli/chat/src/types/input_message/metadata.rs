use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputMessageMetadata {
    #[serde(rename = "traceparent")]
    pub traceparent: String,

    #[serde(rename = "#uniqueId")]
    pub unique_id: String,

    #[serde(rename = "#date_processed")]
    pub date_processed: String,

    #[serde(rename = "date_created")]
    pub date_created: String,
}

impl InputMessageMetadata {
    pub fn new() -> Self {
        Self {
            traceparent: "00-95e56e83e046651a2bb9333a77026ccc-077587ba8f7f15cd-01".to_string(),
            unique_id: "de88b11b-c4e9-4f0d-bf7a-4cec4925f753".to_string(),
            date_processed: "1752061607831".to_string(),
            date_created: "1752061607840".to_string(),
        }
    }
}
