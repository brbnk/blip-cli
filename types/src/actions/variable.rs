use contexts::{context, replacer};
use serde::{Deserialize, Serialize};

use crate::actions::print::print_red;

#[derive(Debug, Serialize, Deserialize)]
pub struct Variable {
    #[serde(rename = "variable")]
    pub variable: String,

    #[serde(rename = "value")]
    pub value: String
}

impl Variable {
    pub fn execute(&self) {
        let replaced = replacer::replace(&self.value);

        print_red(
            "SetVariable", 
            &self.variable, 
            &replaced);

        context::set(&self.variable, replaced.as_str());
    }
}