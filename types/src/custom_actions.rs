use serde::{Serialize, Deserialize};
use crate::conditions::Condition;
use crate::actions::{
    Script,
    Variable,
    ProcessHttp,
    MergeContact,
    Redirect,
    ScriptV2,
    ProcessCommand,
    ExecuteBlipFunction,
    ProcessContentAssistant,
    TrackEvent
};

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
            Settings::Script(script) => script.execute(),
            Settings::Variable(variable) => variable.execute(),
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