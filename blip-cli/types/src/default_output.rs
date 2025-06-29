use contexts::replacer;
use serde::{Serialize,Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultOutput {
    #[serde(rename = "stateId")]
    pub state_id: String,

    #[serde(rename = "typeOfStateId")]
    pub state_type: Option<StateType>
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StateType {
    Variable,
    State
}

impl DefaultOutput {
    pub fn get_state_id(&self) -> String {
        if let Some(state_type) = &self.state_type {
            match state_type {
                StateType::Variable => replacer::replace(&self.state_id),
                StateType::State => self.state_id.clone(),
            }
        }
        else {
            self.state_id.clone()
        }
    }
}