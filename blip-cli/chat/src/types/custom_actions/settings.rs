use serde::{Serialize, Deserialize};
use domain::traits::chat::Executable;
use crate::types::actions::{
    ExecuteBlipFunction, 
    MergeContact, 
    ProcessCommand, 
    ProcessContentAssistant, 
    ProcessHttp, 
    Redirect, 
    Script, 
    ScriptV2, 
    TrackEvent, 
    Variable
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Settings {
    Script(Script),
    Variable(Variable),
    ProcessHttp(ProcessHttp),
    TrackEvent(TrackEvent),
    MergeContact(MergeContact),
    Redirect(Redirect),
    ScriptV2(ScriptV2),
    ProcessCommand(ProcessCommand),
    ExecuteBlipFunction(ExecuteBlipFunction),
    ProcessContentAssistant(ProcessContentAssistant)
}

impl Settings {
    pub fn as_executable(&self) -> &dyn Executable {
        match self {
            Settings::Script(script) => script,
            Settings::Variable(variable) => variable,
            Settings::ProcessHttp(process_http) => process_http,
            Settings::TrackEvent(track_event) => track_event,
            Settings::MergeContact(merge_contact) => merge_contact,
            Settings::Redirect(redirect) => redirect,
            Settings::ScriptV2(script_v2) => script_v2,
            Settings::ProcessCommand(process_command) => process_command,
            Settings::ExecuteBlipFunction(execute_blip_function) => execute_blip_function,
            Settings::ProcessContentAssistant(process_content_assistant) => process_content_assistant,
        }
    }
}
