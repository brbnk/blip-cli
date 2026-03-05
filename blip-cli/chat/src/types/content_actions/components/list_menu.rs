use serde::{Deserialize, Serialize};

use crate::content_actions::components::{QuickReplyOption};

#[derive(Debug, Serialize, Deserialize)]
pub struct ListMenu {
    #[serde(rename = "text")]
    pub text: String,

    #[serde(rename = "limitMenu")]
    pub limit_menu: bool,

    #[serde(rename = "options")]
    pub options: Vec<QuickReplyOption>,
}