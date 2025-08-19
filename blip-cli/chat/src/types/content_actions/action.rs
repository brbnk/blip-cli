use serde::{Deserialize, Serialize};
use super::{Settings, CardContent};

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    #[serde(rename = "$id")]
    pub id: String,

    #[serde(rename = "type")]
    pub action_type: String,

    #[serde(rename = "settings")]
    pub settings: Settings,

    #[serde(rename = "$cardContent")]
    pub card_content: Option<CardContent>,
}

impl Action {
    pub fn handle_card_content(&self) {
        match &self.card_content {
            Some(content) => content.handle_document(),
            None => {},
        }
    }
}