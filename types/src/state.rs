use crate::condition_outputs::ConditionOutputs;
use crate::content::ContentAction;
use crate::custom_actions::CustomAction;
use crate::default_output::DefaultOutput;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
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
    pub title: String,
}

impl State {
    pub fn handle_content_actions(&self) {
        for content in &self.content_actions {
            match content {
                ContentAction::Action { action } => {
                    action.handle_action();
                }
                ContentAction::Input { input } => {
                    input.handle_input();
                }
            }
        }
    }
}
