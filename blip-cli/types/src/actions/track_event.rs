use std::{collections::HashMap};
use contexts::replacer;
use serde::{Deserialize, Serialize};

use crate::actions::{Executable};

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
        ui::printer::print_blue(
          "Tracking", 
          &replacer::replace(&self.category), 
          &replacer::replace(&self.action));
    }
}