use serde::{Serialize,Deserialize};
use crate::content::ContentAction;
use crate::custom_actions::CustomAction;
use crate::default_output::DefaultOutput;
use crate::condition_outputs::ConditionOutputs;

#[derive(Serialize,Deserialize,Debug)]
pub struct State {
    #[serde(rename = "$contentActions")]
    pub content_actions: Vec<ContentAction>,

    #[serde(rename = "$conditionOutputs")]
    pub condition_outputs: Vec<ConditionOutputs>,
    
    #[serde(rename = "$enteringCustomActions")]
    pub entering_custom_actions: Vec<CustomAction>,

    #[serde(rename = "$leavingCustomActions")]
    pub leaving_custom_actions: Vec<CustomAction>,

    #[serde(rename = "$defaultOutput")]
    pub default_output: DefaultOutput,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "$title")]
    pub title: String
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

    pub fn display_title(&self) {
        println!("{}", self.id);
    }
}
