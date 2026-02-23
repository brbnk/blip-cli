use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentMessage {
    #[serde(rename = "content")]
    pub content: Content
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    #[serde(rename = "type")]
    pub content_type: String,

    #[serde(rename = "value")]
    pub value: Value
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Value {
    #[serde(rename = "type")]
    pub value_type: String
}

impl AgentMessage {
    pub fn new() -> Self {
        Self {
            content: Content {
                content_type: "application/vnd.iris.aiplatform.handoff+json".to_string(),
                value: Value {
                    value_type: "outputForward".to_string(),
                }
            }
        }
    }
}
