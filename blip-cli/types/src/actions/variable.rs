use contexts::{context, replacer};
use serde::{Deserialize, Serialize};

use crate::actions::{Executable};

#[derive(Debug, Serialize, Deserialize)]
pub struct Variable {
    #[serde(rename = "variable")]
    pub variable: String,

    #[serde(rename = "value")]
    pub value: String
}

impl Executable for Variable {
    fn execute(&self) {
        let replaced = replacer::replace(&self.value);

        ui::printer::print_red(
            "SetVariable", 
            &self.variable, 
            &replaced);

        context::set(&self.variable, replaced.as_str());
    }
}