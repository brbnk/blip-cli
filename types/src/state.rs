use crate::{condition_outputs::ConditionOutputs, global_actions::GlobalActions};
use crate::content::ContentAction;
use crate::custom_actions::CustomAction;
use crate::default_output::DefaultOutput;
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
    pub fn print_state_title(&self) {
        let title = &self.title;
        let length = 60;
        let min_length = title.len() + 2;
        let length = length.max(min_length);

        let upper_state = format!("+{}+", "-".repeat(length - 2));
        let total_padding = length - 2 - title.len();
        let left_padding = total_padding / 2;
        let right_padding = total_padding - left_padding;

        println!("{}", upper_state.bright_black());
        println!(
            "{}{}{}{}{}",
            "|".bright_black(),
            " ".repeat(left_padding),
            title.green().bold(),
            " ".repeat(right_padding),
            "|".bright_black()
        );
        println!("{}", upper_state.bright_black());
    }

    pub fn handle_global_leaving_actions(&self, is_first_input: bool) {
        let has_input = self.content_actions.iter().any(|ca| {
            match ca {
                ContentAction::Action {action: _} => false,
                ContentAction::Input { input } => !input.bypass,
            }
        });

        if has_input && !is_first_input {
            let global_actions = GlobalActions::deserialize(&context::get_master_state());
            global_actions.handle_custom_leaving_actions();
            print!("\n");
        }
    }

    pub fn save_previous(&self) {
        context::set("prvious.state.id", &self.id);
        context::set("prvious.state.name", &self.title);
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
        println!();
        for action in &self.leaving_custom_actions {
            if action.should_execute() {
                action.execute();
            }
        }
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

    pub fn get_default_output(&self) -> &String {
        &self.default_output.state_id
    }
}
