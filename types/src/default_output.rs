use serde::{Serialize,Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultOutput {
    #[serde(rename = "stateId")]
    pub state_id: String
}