use super::content_actions::ContentAction;
use contexts::global_actions;
use serde::{Deserialize, Serialize};

use super::{
    condition_outputs::ConditionOutputs, custom_actions::CustomAction,
    default_output::DefaultOutput,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct GlobalActions {
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
}

impl GlobalActions {
    pub fn deserialize(identifier: &str) -> GlobalActions {
        let result: GlobalActions = match global_actions::get(identifier) {
            Some(serialized_flow) => json_converter::deserialize(&serialized_flow)
                .expect("Falha ao desserializar o parse do fluxo!"),
            None => panic!(),
        };
        result
    }

    pub fn handle_custom_entering_actions(&self) {
        for action in &self.entering_custom_actions {
            if action.should_execute() {
                action.execute();
            }
        }
    }

    pub fn handle_custom_leaving_actions(&self) {
        for action in &self.leaving_custom_actions {
            if action.should_execute() {
                action.execute();
            }
        }
    }
}
