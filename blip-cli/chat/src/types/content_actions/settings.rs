use serde::{Deserialize, Serialize};
use crate::content_actions::DynamicContent;

use super::Content;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Settings {
    Default(SettingsDefault),
    DynamicContent(DynamicContent)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsDefault {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "type")]
    pub settings_type: String,

    #[serde(rename = "content")]
    pub content: Content,
}