use std::{collections::HashMap};
use contexts::{replacer, system, MANAGER_POOL};
use serde::{Deserialize, Serialize};

use domain::chat::Executable;
use ui::{printer, types::{ActionProps, Color}};

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackEvent {
    #[serde(rename = "extras")]
    pub extras: Option<HashMap<String, String>>,

    #[serde(rename = "category")]
    pub category: String,

    #[serde(rename = "action")]
    pub action: String,

    #[serde(rename = "subflowDefaultAction")]
    pub subflow_default_action: Option<bool>
}

impl Executable for TrackEvent {
    fn execute(&self) {
        if !system::is_test_mode() {
            printer::print_action(ActionProps {
                name: String::from("Tracking"),
                key: String::from(replacer::replace(&self.category)),
                value: String::from(replacer::replace(&self.action)),
                color: Color::Blue,
            });
        } else {
            let event = replacer::replace(&serde_json::to_string(&self).expect("track event serialized"));
            MANAGER_POOL.event.set(&system::get_master_state(), &event);
        }
    }
}