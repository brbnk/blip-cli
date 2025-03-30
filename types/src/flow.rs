use serde::{Serialize,Deserialize};
use crate::content::ContentAction;
use crate::custom_actions::CustomAction;

#[derive(Serialize,Deserialize,Debug)]
pub struct State {
    #[serde(rename = "$contentActions")]
    pub content_actions: Vec<ContentAction>,
    
    #[serde(rename = "$enteringCustomActions")]
    pub entering_custom_actions: Vec<CustomAction>,

    #[serde(rename = "$leavingCustomActions")]
    pub leaving_custom_actions: Vec<CustomAction>
}

impl State {
    pub fn list_content_actions(&self) {
        for item in &self.content_actions {
            match item {
                ContentAction::Action { action } => {
                    println!("{}", action.id);
                }
                ContentAction::Input { input } => {
                    println!("{}", input.card_content.document.id);
                }
            }
        }
    }
}
