use serde::{Serialize, Deserialize};
use domain::chat::Executable;
use crate::{actions::ForwardToDesk, types::actions::{
    Agent, ExecuteBlipFunction, MergeContact, ProcessCommand, ProcessContentAssistant, ProcessHttp, Redirect, Script, ScriptV2, TrackEvent, Variable
}};

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
    ProcessContentAssistant(ProcessContentAssistant),
    ForwardToDesk(ForwardToDesk),
    Agent(Agent)
}

impl Settings {
    pub fn as_executable(&self) -> &dyn Executable {
        match self {
            Settings::Script(s) => s,
            Settings::Variable(v) => v,
            Settings::ProcessHttp(ph) => ph,
            Settings::TrackEvent(te) => te,
            Settings::MergeContact(mc) => mc,
            Settings::Redirect(r) => r,
            Settings::ScriptV2(sv) => sv,
            Settings::ProcessCommand(pc) => pc,
            Settings::ExecuteBlipFunction(ebf) => ebf,
            Settings::ProcessContentAssistant(pca) => pca,
            Settings::Agent(a) => a,
            Settings::ForwardToDesk(ftd) => ftd,
        }
    }
}
