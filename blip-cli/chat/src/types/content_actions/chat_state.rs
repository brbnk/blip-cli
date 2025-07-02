use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatState {
    #[serde(rename = "state")]
    pub state: String,

    #[serde(rename = "interval")]
    pub interval: u32,
}