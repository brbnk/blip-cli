use contexts::{replacer, system};
use serde::{Deserialize, Serialize};
use ui::{printer, types::Color};
use crate::content_actions::DynamicContent;

use super::Content;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Document {
    Default(DocumentContent),
    DynamicContent(DynamicContent)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentContent {
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
        if !system::is_test_mode() {
            match &self {
                Document::Default(document_content) => {
                    match &document_content.content {
                        Some(content) => {
                            match content {
                                Content::ChatState(json) => {
                                    let animation_time = (json.interval / 1000) as u32;
                                    ui::loader::start(animation_time);
                                }
                                Content::Text(text) => {
                                    printer::println(&replacer::replace(&text), Color::Yellow);
                                    println!();
                                }
                                Content::Media(media) => {
                                    let serialized = serde_json::to_string(media).unwrap_or(String::from("media content"));
                                    printer::println(&replacer::replace(&serialized), Color::Yellow);
                                    println!();
                                }
                            }
                        },
                        None => {
                            println!("Nenhum conteúdo encontrado!")
                        }
                    }
                },
                Document::DynamicContent(dynamic_content) => {
                    let serialized = serde_json::to_string(dynamic_content).unwrap_or(String::from("dynamic content"));
                    printer::println(&replacer::replace(&serialized), Color::Yellow);
                    println!();
                },
            }
        }
    }
}