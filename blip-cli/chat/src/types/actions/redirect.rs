use contexts::{replacer, system, MANAGER_POOL};
use serde::{Deserialize, Serialize};

use domain::chat::Executable;
use ui::{printer, types::{ActionProps, Color}};

#[derive(Debug, Serialize, Deserialize)]
pub struct Redirect {
    #[serde(rename = "context")]
    pub context: Option<RedirectContext>,

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
        let redirect = replacer::replace(
            &serde_json::to_string(&self).expect("redirect event"));
       
        system::set_redirect(&redirect);
        
        if !system::is_test_mode() {
            let value = match &self.context {
                Some(c) => replacer::replace(&c.value),
                None => String::from("no_context"),
            };

            printer::print_action(ActionProps {
                name: String::from("Redirect"),
                key: String::from("Service"),
                value: format!("{} ({})", replacer::replace(&self.address), &value),
                color: Color::Blue,
            });
        } else {    
            let event = replacer::replace(&serde_json::to_string(&self).expect("redirect event serialized"));
            MANAGER_POOL.event.set(&system::get_master_state(), &event);
        }
    }
}