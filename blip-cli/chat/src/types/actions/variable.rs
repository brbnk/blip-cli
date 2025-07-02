use contexts::{context, replacer};
use serde::{Deserialize, Serialize};

use domain::traits::chat::Executable;
use ui::{printer, types::{ActionProps, Color}};

#[derive(Debug, Serialize, Deserialize)]
pub struct Variable {
    #[serde(rename = "variable")]
    pub variable: String,

    #[serde(rename = "value")]
    pub value: Option<String>
}

impl Executable for Variable {
    fn execute(&self) {
        let replaced = match &self.value {
            Some(v) => replacer::replace(v),
            None => "".to_string(),
        };

        context::set(&self.variable, replaced.as_str());

        printer::print_action(ActionProps {
            name: String::from("SetVariable"),
            key: String::from(&self.variable),
            value: String::from(&replaced),
            color: Color::Red,
        });
    }
}