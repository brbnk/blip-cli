use contexts::replacer;
use serde::{Deserialize, Serialize};
use ui::{printer, types::Color};
use super::Content;

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "type")]
    pub doc_type: String,

    #[serde(rename = "textContent")]
    pub text_content: Option<String>,

    #[serde(rename = "content")]
    pub content: Option<Content>,
}

impl Document {
    pub fn handle_content(&self) {
        match &self.content {
            Some(Content::ChatState(json)) => {
                let animation_time = (json.interval / 1000) as u32;
                ui::loader::start(animation_time);
            }
            Some(Content::Text(text)) => {
                printer::println(&replacer::replace(&text), Color::Yellow);
                println!();
            }
            None => {
                println!("Nenhum conteúdo encontrado!");
            }
        }
    }
}