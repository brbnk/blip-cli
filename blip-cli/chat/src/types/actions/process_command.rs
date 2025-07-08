use contexts::{replacer, system, MANAGER_POOL};
use serde::{Deserialize, Serialize};

use domain::traits::chat::Executable;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessCommand {
    #[serde(rename = "to")]
    pub to: String,

    #[serde(rename = "method")]
    pub method: String,

    #[serde(rename = "uri")]
    pub uri: String,

    #[serde(rename = "variable")]
    pub variable: String
}

impl Executable for ProcessCommand {
    fn execute(&self) {
        if system::is_test_mode() {
            let event = replacer::replace(&serde_json::to_string(&self).expect("process command event serialized"));
            MANAGER_POOL.event.set(&system::get_master_state(), &event);
        }
    }
}