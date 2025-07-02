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
    pub card_content: CardContent,
}

impl Action {
    pub fn handle_card_content(&self) {
        self.card_content.handle_document();
    }
}