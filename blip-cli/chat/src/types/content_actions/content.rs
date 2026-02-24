use serde::{Deserialize, Serialize};
use crate::content_actions::{MediaContent, components::QuickReply};
use super::ChatState;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Content {
    Text(String),
    ChatState(ChatState),
    Media(MediaContent),
    QuickReply(QuickReply)
}