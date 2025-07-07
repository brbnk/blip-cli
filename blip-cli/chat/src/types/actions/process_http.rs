use std::collections::HashMap;

use contexts::{replacer, system, MANAGER_POOL};
use serde::{Deserialize, Serialize};

use domain::traits::chat::Executable;

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
        let event = replacer::replace(&serde_json::to_string(&self).expect("process http event serialized"));
        MANAGER_POOL.event.set(&system::get_master_state(), &event);
    }
}