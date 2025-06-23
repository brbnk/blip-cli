use crate::condition_outputs::ConditionOutputs;
use crate::content::ContentAction;
use crate::custom_actions::CustomAction;
use crate::default_output::DefaultOutput;
use colored::Colorize;
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
        let texto = &self.title;
        let largura = 60;
        let largura_minima = texto.len() + 2;
        let largura = largura.max(largura_minima);

        let topo_base = format!("+{}+", "-".repeat(largura - 2));
        let padding_total = largura - 2 - texto.len();
        let padding_esquerda = padding_total / 2;
        let padding_direita = padding_total - padding_esquerda;

        println!("{}", topo_base);
        println!(
            "|{}{}{}|",
            " ".repeat(padding_esquerda),
            texto.green().bold(),
            " ".repeat(padding_direita)
        );
        println!("{}", topo_base);
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

    pub fn handle_condition_outputs(&self) -> Option<&String> {
        for condition_output in &self.condition_outputs {
            match condition_output.get_destination() {
                Some(destination) => {
                    return Some(destination)
                },
                None => None::<&String>,
            };
        }
        return None;
    }

    pub fn get_default_output(&self) -> &String {
        &self.default_output.state_id
    }
}
