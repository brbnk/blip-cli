use super::CardContent;
use contexts::{test, MANAGER_POOL};
use serde::{Deserialize, Serialize};
use std::{
    collections::VecDeque,
    io::{self, Write},
};
use ui::{printer, types::Color};

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
        if test::is_activated() {
            input_content = self.handle_test_input();
        } else {
            io::stdout().flush().unwrap();

            let bytes_read = io::stdin()
                .read_line(&mut input_content)
                .expect("Erro ao ler entrada");

            ui::loader::start(1);

            if bytes_read == 0 {
                std::process::exit(0);
            }

            input_content = input_content.trim().to_string();
        }

        MANAGER_POOL.context.set("input.content", &input_content);
    }

    fn handle_test_input(&self) -> String {
        match test::get_inputs() {
            Some(i) => {
                let mut inputs: VecDeque<String> = serde_json::from_str(&i).expect("vec of inputs");

                let result = match inputs.pop_front() {
                    Some(text) => {
                        println!("{}", &text);
                        text
                    }
                    None => {
                        test::set_end_signal();
                        "".to_string()
                    },
                };

                test::set_inputs(&serde_json::to_string(&inputs).expect("vec stringified"));

                return result;
            }
            None => test::set_end_signal(),
        };

        test::set_end_signal();
        return "".to_string();
    }
}
