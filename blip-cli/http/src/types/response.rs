use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct ProxyResponse {
    #[serde(rename = "success")]
    pub sucesss: bool,

    #[serde(rename = "data")]
    pub data: Value,

    #[serde(rename = "message")]
    pub message: Option<String>,

    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>
}