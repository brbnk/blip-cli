use crate::content::ContentAction;
use crate::custom_actions::CustomAction;
use crate::default_output::DefaultOutput;
use crate::{condition_outputs::ConditionOutputs, global_actions::GlobalActions};
use colored::Colorize;
use contexts::context;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    #[serde(rename = "$enteringCustomActions")]
    pub entering_custom_actions: Vec<CustomAction>,

    #[serde(rename = "$contentActions")]
    pub content_actions: Vec<ContentAction>,

    #[serde(rename = "$conditionOutputs")]
    pub condition_outputs: Vec<ConditionOutputs>,

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
    pub fn handle_global_leaving_actions(&self, is_first_input: bool) {
        if self.has_input() && !is_first_input {
            let global_actions = GlobalActions::deserialize(&context::get_master_state());
            global_actions.handle_custom_leaving_actions();
        }
    }

    pub fn save_previous(&self) {
        context::set("state.previous.id", &self.id);
        context::set("state.previous.name", &self.title);
    }

    pub fn save_current(&self) {
        context::set("state.id", &self.id);
        context::set("state.name", &self.title);
    }

    pub fn handle_custom_entering_actions(&self) {
        for action in &self.entering_custom_actions {
            if action.should_execute() {
                action.execute();
            }
        }
        println!();
    }

    pub fn handle_custom_leaving_actions(&self) {
        for action in &self.leaving_custom_actions {
            if action.should_execute() {
                action.execute();
            }
        }
        let length = 60;
        let bottom_state = format!("|{}|", "_".repeat(length - 2));
        println!("{}", bottom_state.bright_black());
    }

    pub fn handle_content_actions(&self, is_first_input: bool) {
        for content in &self.content_actions {
            match content {
                ContentAction::Action { action } => {
                    action.handle_action();
                }
                ContentAction::Input { input } => {
                    input.handle_input();
                    if !is_first_input && !input.bypass {
                        print!("\n");
                        let global_actions = GlobalActions::deserialize(&context::get_master_state());
                        global_actions.handle_custom_entering_actions();
                    }
                }
            }
        }
    }

    pub fn handle_condition_outputs(&self) -> Option<&String> {
        for condition_output in &self.condition_outputs {
            let destination = condition_output.get_destination();
            if destination.is_some() {
                return Some(destination.unwrap());
            }
        }
        None
    }

    pub fn get_default_output(&self) -> String {
        self.default_output.get_state_id()
    }

    pub fn has_input(&self) -> bool {
        self.content_actions.iter().any(|ca| match ca {
            ContentAction::Action { action: _ } => false,
            ContentAction::Input { input } => !input.bypass,
        })
    }
}
