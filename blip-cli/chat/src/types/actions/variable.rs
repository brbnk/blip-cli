use contexts::{replacer, system, MANAGER_POOL};
use serde::{Deserialize, Serialize};

use domain::{chat::Executable};
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

        MANAGER_POOL.context.set(&self.variable, &replaced);

        if !system::is_test_mode() {
            printer::print_action(ActionProps {
                name: String::from("SetVariable"),
                key: String::from(replacer::replace(&self.variable)),
                value: String::from(&replaced),
                color: Color::Red,
            });
        } else {
            let event = replacer::replace(&serde_json::to_string(&self).expect("variable serialized"));
            MANAGER_POOL.event.set(&system::get_master_state(), &event);
        }
    }
}