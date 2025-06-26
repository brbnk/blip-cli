use colored::Colorize;
use contexts::{context, replacer};
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
                // json.interval / 1000
                let frames = ["|", "/", "-", "\\"];
                let animation_time = (json.interval / 100) as usize;
                for i in 0..animation_time {
                    let frame = frames[i % frames.len()];
                    print!("\r{}", frame);
                    std::io::Write::flush(&mut std::io::stdout()).unwrap();
                    sleep(Duration::from_millis(100));
                }
                print!("\x08 \x08");
            }
            Some(Content::Text(text)) => {
                print!("{}\n", replacer::replace(text).white().bold());
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
        if self.bypass {
            return;
        }

        let mut input_content = String::new();

        print!("\n\n> ");
        io::stdout().flush().unwrap();
        
        let bytes_read = io::stdin()
            .read_line(&mut input_content)
            .expect("Erro ao ler entrada");
        print!("\n");
        if bytes_read == 0 {
            std::process::exit(0);
        }

        let input_content = input_content.trim().to_string();

        context::set("input.content", &input_content);
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

    #[serde(rename = "textContent")]
    pub text_content: Option<String>,

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
