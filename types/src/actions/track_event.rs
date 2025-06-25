use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::actions::print::print_blue;

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackEvent {
    #[serde(rename = "extras")]
    pub extras: HashMap<String, String>,

    #[serde(rename = "category")]
    pub category: String,

    #[serde(rename = "action")]
    pub action: String
}

impl TrackEvent {
    pub fn execute(&self) {
        print_blue(
          "Tracking", 
          &self.category, 
          &self.action);
    }
}