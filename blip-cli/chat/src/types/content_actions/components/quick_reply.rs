use serde::{Deserialize, Serialize};

use crate::content_actions::components::{QuickReplyOption};

#[derive(Debug, Serialize, Deserialize)]
pub struct QuickReply {
    #[serde(rename = "text")]
    pub text: String,

    #[serde(rename = "scope")]
    pub scope: String,

    #[serde(rename = "options")]
    pub options: Vec<QuickReplyOption>,
}