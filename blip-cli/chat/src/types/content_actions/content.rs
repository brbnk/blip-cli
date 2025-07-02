use serde::{Deserialize, Serialize};
use super::ChatState;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Content {
    Text(String),
    ChatState(ChatState),
}