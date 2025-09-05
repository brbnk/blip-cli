use crate::{actions::Redirect, types::input_message::InputMessage};

use super::CardContent;
use contexts::{replacer, system, MANAGER_POOL};
use file_handler::deserialize;
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

        if system::has_redirect() {
            system::set_redirect_signal();
            let serialized = system::get_redirect().expect("serialized redirect event");
            let redirect: Redirect = deserialize(&serialized).expect("deserialized redirect event");
            
            let input_content = match &redirect.context {
                Some(c) => replacer::replace(&c.value),
                None => "".to_owned(),
            };

            MANAGER_POOL.context.set("input.content", &input_content);
            MANAGER_POOL.context.set("input.message", &serde_json::to_string(&InputMessage::new(&input_content)).expect("serialized input message"))
        } 
        else {
            if system::is_redirect_transition_signal() {
                system::reset_redirect_transition_signal();
            }
            else {
                let mut input_content = String::new();

                if system::is_test_mode() {
                    input_content = self.handle_test_input();
                }
                else {
                    printer::print("> ", Color::Green);
                    
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
                MANAGER_POOL.context.set("input.message", &serde_json::to_string(&InputMessage::new(&input_content)).expect("serialized input message"))
            }
        }
    }

    fn handle_test_input(&self) -> String {
        match system::get_test_inputs() {
            Some(i) => {
                let mut inputs: VecDeque<String> = serde_json::from_str(&i).expect("vec of inputs");

                let result = match inputs.pop_front() {
                    Some(text) => {
                        text
                    }
                    None => {
                        system::set_end_test_signal();
                        "".to_string()
                    },
                };

                system::set_test_inputs(&serde_json::to_string(&inputs).expect("vec stringified"));

                return result;
            }
            None => system::set_end_test_signal(),
        };

        system::set_end_test_signal();
        return "".to_string();
    }
}
