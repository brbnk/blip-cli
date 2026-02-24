use contexts::{MANAGER_POOL};
use domain::chat::Executable;
use serde::{Deserialize, Serialize};

use crate::types::input_message::{AgentMessage, InputMessage};

#[derive(Debug, Serialize, Deserialize)]
pub struct Agent {
    #[serde(rename = "output")]
    pub output: Output,
}

impl Executable for Agent {
    fn execute(&self) {
        if !self.output.forward.enabled {
            let input_content = serde_json::to_string(&AgentMessage::new()).expect("serialized agent message");
            MANAGER_POOL.context.set("input.content", &input_content);
            MANAGER_POOL.context.set("input.message", &serde_json::to_string(&InputMessage::new(&input_content)).expect("serialized input message"))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    #[serde(rename = "forward")]
    pub forward: Forward
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Forward {
    #[serde(rename = "enabled")]
    pub enabled: bool,

    #[serde(rename = "outputVariable")]
    pub output_variable: String,

    #[serde(rename = "handoffName")]
    pub handoff_name: String,
}