use serde::{Serialize, Deserialize};

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