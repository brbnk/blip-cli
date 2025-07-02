use serde::{Deserialize, Serialize};
use ui::{printer, types::Color};
use std::io::{self, Write};
use contexts::context;
use super::CardContent;

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

        printer::print("> ", Color::Green);
        io::stdout().flush().unwrap();
        
        let bytes_read = io::stdin()
            .read_line(&mut input_content)
            .expect("Erro ao ler entrada");

        ui::loader::start(1);

        if bytes_read == 0 {
            std::process::exit(0);
        }

        let input_content = input_content.trim().to_string();

        context::set("input.content", &input_content);
    }
}