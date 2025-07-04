use std::{collections::HashMap};
use contexts::replacer;
use serde::{Deserialize, Serialize};

use domain::traits::chat::Executable;
use ui::{printer, types::{ActionProps, Color}};

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackEvent {
    #[serde(rename = "extras")]
    pub extras: HashMap<String, String>,

    #[serde(rename = "category")]
    pub category: String,

    #[serde(rename = "action")]
    pub action: String
}

impl Executable for TrackEvent {
    fn execute(&self) {
        printer::print_action(ActionProps {
            name: String::from("Tracking"),
            key: String::from(replacer::replace(&self.category)),
            value: String::from(replacer::replace(&self.action)),
            color: Color::Blue,
        });
    }
}