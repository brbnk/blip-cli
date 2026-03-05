use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct BlipFunctionsResult {
    #[serde(rename = "tenant")]
    pub tenant: Option<String>,

    #[serde(rename = "functions")]
    pub functions: Vec<Value>,
}