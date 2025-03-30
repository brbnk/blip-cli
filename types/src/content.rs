use serde::{Serialize,Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContentAction {
    Action { action: Action },
    Input { input: Input }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    #[serde(rename = "$id")]
    pub id: String,

    #[serde(rename = "type")]
    pub action_type: String,
    
    #[serde(rename = "settings")]
    pub settings: Settings,
    
    #[serde(rename = "$cardContent")]
    pub card_content: CardContent
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Input {
    #[serde(rename = "$cardContent")]
    pub card_content: CardContent
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "type")]
    pub settings_type: String,

    #[serde(rename = "content")]
    pub content: Content
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardContent {
    #[serde(rename = "document")]
    pub document: Document
}
 
#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "type")]
    pub doc_type: String,

    #[serde(rename = "content")]
    pub content: Content 
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Content {
    Text(String),
    Json(ChatState)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatState {
    #[serde(rename = "state")]
    pub state: String,

    #[serde(rename = "interval")]
    pub interval: u32
}
