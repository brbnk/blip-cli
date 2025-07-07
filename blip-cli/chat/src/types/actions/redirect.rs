use contexts::{replacer, system, MANAGER_POOL};
use serde::{Deserialize, Serialize};

use domain::traits::chat::Executable;

#[derive(Debug, Serialize, Deserialize)]
pub struct Redirect {
    #[serde(rename = "context")]
    pub context: RedirectContext,

    #[serde(rename = "address")]
    pub address: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedirectContext {
    #[serde(rename = "type")]
    pub redirect_type: String,

    #[serde(rename = "value")]
    pub value: String
}

impl Executable for Redirect {
    fn execute(&self) {
        let event = replacer::replace(&serde_json::to_string(&self).expect("redirect event serialized"));
        MANAGER_POOL.event.set(&system::get_master_state(), &event);
    }
}