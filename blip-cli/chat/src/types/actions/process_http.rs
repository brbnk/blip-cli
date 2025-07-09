use std::collections::HashMap;

use contexts::{replacer, store, system, MANAGER_POOL};
use serde::{Deserialize, Serialize};

use domain::chat::Executable;
use ui::{printer, types::{ActionProps, Color}};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessHttp {
    #[serde(rename = "headers")]
    pub headers: HashMap<String, String>,

    #[serde(rename = "method")]
    pub method: String,

    #[serde(rename = "uri")]
    pub uri: String,

    #[serde(rename = "responseStatusVariable")]
    pub status: String,

    #[serde(rename = "responseBodyVariable")]
    pub response: String
}

impl Executable for ProcessHttp {
    fn execute(&self) {
        if !system::is_test_mode() {
            printer::print_action(ActionProps {
                name: String::from("ProcessHttp"),
                key: format!("{} {}", &self.method, replacer::replace(&self.uri)),
                value: format!("Status: {} | Response: {}", store::get(&self.status).unwrap_or("".to_string()), store::get(&self.response).unwrap_or("".to_string())),
                color: Color::Purple,
            });
        } else {
            let event = replacer::replace(&serde_json::to_string(&self).expect("process http event serialized"));
            MANAGER_POOL.event.set(&system::get_master_state(), &event);
        }
    }
}