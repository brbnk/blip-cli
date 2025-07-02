use serde::{Deserialize, Serialize};
use super::Content;

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "type")]
    pub settings_type: String,

    #[serde(rename = "content")]
    pub content: Content,
}