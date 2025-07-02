use serde::{Deserialize, Serialize};
use super::Document;

#[derive(Debug, Serialize, Deserialize)]
pub struct CardContent {
    #[serde(rename = "document")]
    pub document: Document,
}

impl CardContent {
    pub fn handle_document(&self) {
        self.document.handle_content();
    }
}
