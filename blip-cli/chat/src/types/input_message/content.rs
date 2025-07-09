use serde::{Deserialize, Serialize};

use crate::types::input_message::MediaContent;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputMessageContent {
    Text(String),
    Media(MediaContent)
}