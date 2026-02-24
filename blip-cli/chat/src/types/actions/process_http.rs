use std::{collections::HashMap};

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
    pub status: Option<String>,

    #[serde(rename = "responseBodyVariable")]
    pub response: Option<String>
}

impl Executable for ProcessHttp {
    fn execute(&self) {
        if !system::is_test_mode() {
            let status = self.status.clone().unwrap_or(String::from(""));
            let response = self.response.clone().unwrap_or(String::from(""));
            printer::print_action(ActionProps {
                name: String::from("ProcessHttp"),
                key: format!("{} {}", &self.method, replacer::replace(&self.uri)),
                value: format!(
                    "Status: {} | Response: {}", 
                    store::get(&status).unwrap_or(String::from("")), 
                    store::get(&response).unwrap_or(String::from(""))),
                color: Color::Purple,
            });
        } else {
            let event = replacer::replace(&serde_json::to_string(&self).expect("process http event serialized"));
            MANAGER_POOL.event.set(&system::get_master_state(), &event);
        }
    }
}