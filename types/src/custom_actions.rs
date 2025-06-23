use std::collections::HashMap;
use colored::Colorize;
use serde::{Serialize, Deserialize};
use crate::conditions::Condition;


#[derive(Debug, Serialize, Deserialize)]
pub struct Script {
    #[serde(rename = "function")]
    pub function: String,

    #[serde(rename = "source")]
    pub source: String,

    #[serde(rename = "inputVariables")]
    pub input_variables: Vec<String>,

    #[serde(rename = "outputVariable")]
    pub output_variable: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Variable {
    #[serde(rename = "variable")]
    pub variable: String,

    #[serde(rename = "value")]
    pub value: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessHttp {
    #[serde(rename = "headers")]
    pub headers: HashMap<String, String>,

    #[serde(rename = "method")]
    pub method: String,

    #[serde(rename = "uri")]
    pub uri: String,

    #[serde(rename = "responseStatusVariable")]
    pub status: String,

    #[serde(rename = "responseBodyVariable")]
    pub response: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MergeContact {
    #[serde(rename = "extras")]
    pub extras: HashMap<String, String>,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "email")]
    pub email: String,

    #[serde(rename = "city")]
    pub city: String,

    #[serde(rename = "phoneNumber")]
    pub phone_number: String,

    #[serde(rename = "taxDocument")]
    pub tax_document: String,

    #[serde(rename = "gender")]
    pub gender: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Redirect {
    #[serde(rename = "context")]
    pub context: RedirectContext,

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

#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptV2 {
    #[serde(rename = "function")]
    pub function: String,

    #[serde(rename = "source")]
    pub source: String,

    #[serde(rename = "inputVariables")]
    pub input_variables: Vec<String>,

    #[serde(rename = "outputVariable")]
    pub output_variable: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessCommand {
    #[serde(rename = "to")]
    pub to: String,

    #[serde(rename = "method")]
    pub method: String,

    #[serde(rename = "uri")]
    pub uri: String,

    #[serde(rename = "variable")]
    pub variable: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecuteBlipFunction {
    #[serde(rename = "function")]
    pub function: String,

    #[serde(rename = "source")]
    pub source: String,

    #[serde(rename = "inputVariables")]
    pub input_variables: Vec<String>,

    #[serde(rename = "outputVariable")]
    pub output_variable: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessContentAssistant {
    #[serde(rename = "v2")]
    pub v2: bool,

    #[serde(rename = "text")]
    pub text: String,

    #[serde(rename = "score")]
    pub score: String,

    #[serde(rename = "outputVariable")]
    pub output_variable: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackEvent {
    #[serde(rename = "extras")]
    pub extras: HashMap<String, String>,

    #[serde(rename = "category")]
    pub category: String,

    #[serde(rename = "action")]
    pub action: String
}

impl TrackEvent {
    pub fn execute(&self) {
        println!(
            "+ {}: {} -> {}", 
            "Tracking".blue().bold(),
            contexts::replacer::replace(&self.category), 
            contexts::replacer::replace(&self.action));
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ActionType {
    ExecuteScript,
    SetVariable,
    ProcessHttp,
    MergeContact,
    Redirect,
    ExecuteScriptV2,
    ProcessCommand,
    ExecuteBlipFunction,
    ProcessContentAssistant,
    TrackEvent
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Settings {
    Script(Script),
    Variable(Variable),
    ProcessHttp(ProcessHttp),
    MergeContact(MergeContact),
    Redirect(Redirect),
    ScriptV2(ScriptV2),
    ProcessCommand(ProcessCommand),
    ExecuteBlipFunction(ExecuteBlipFunction),
    ProcessContentAssistant(ProcessContentAssistant),
    TrackEvent(TrackEvent)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomAction {
    #[serde(rename = "$id")]
    pub id: String,

    #[serde(rename = "type")]
    pub action_type: ActionType,

    #[serde(rename = "$title")]
    pub title: String,

    #[serde(rename = "settings")]
    pub settings: Settings,

    #[serde(rename = "conditions")]
    pub conditions: Option<Vec<Condition>>
}

impl CustomAction {
    pub fn should_execute(&self) -> bool {
        match &self.conditions {
            Some(conditions) => {
                if conditions.is_empty() {
                    return true
                }

                return conditions
                    .iter()
                    .any(|c| c.should_execute())
            },
            None => true,
        }
    }

    pub fn execute(&self) {
        match &self.settings {
            Settings::Script(_) => todo!(),
            Settings::Variable(_) => todo!(),
            Settings::ProcessHttp(_) => todo!(),
            Settings::MergeContact(_) => todo!(),
            Settings::Redirect(_) => todo!(),
            Settings::ScriptV2(_) => todo!(),
            Settings::ProcessCommand(_) => todo!(),
            Settings::ExecuteBlipFunction(_) => todo!(),
            Settings::ProcessContentAssistant(_) => todo!(),
            Settings::TrackEvent(track_event) => track_event.execute(),
        }
    }
}