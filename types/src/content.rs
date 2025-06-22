use serde::{Deserialize, Serialize};
use std::{
    io::{self, Write},
    thread::sleep,
    time::Duration,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContentAction {
    Action { action: Action },
    Input { input: Input },
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
    pub card_content: CardContent,
}

impl Action {
    pub fn handle_action(&self) {
        match &self.card_content.document.content {
            Some(Content::ChatState(json)) => {
                println!("\n... ({}s)", json.interval / 1000);
                sleep(Duration::from_millis(json.interval.into()));
            }
            Some(Content::Text(text)) => {
                println!("\n[BOT]");
                println!("{}", text);
            }
            None => {
                println!("Nenhum conteúdo encontrado!");
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Input {
    #[serde(rename = "bypass")]
    pub bypass: bool,

    #[serde(rename = "$cardContent")]
    pub card_content: CardContent,
}

impl Input {
    pub fn handle_input(&self) {
        io::stdout().flush().unwrap();

        println!("\n[USER]");
        let mut input_content = String::new();
        io::stdin()
            .read_line(&mut input_content)
            .expect("Erro ao ler entrada");

        // let input_content = input_content.trim().to_string();

        // user_context.insert(String::from("input.content"), input_content);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "type")]
    pub settings_type: String,

    #[serde(rename = "content")]
    pub content: Content,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardContent {
    #[serde(rename = "document")]
    pub document: Document,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "type")]
    pub doc_type: String,

    #[serde(rename = "content")]
    pub content: Option<Content>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Content {
    Text(String),
    ChatState(ChatState),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatState {
    #[serde(rename = "state")]
    pub state: String,

    #[serde(rename = "interval")]
    pub interval: u32,
}
