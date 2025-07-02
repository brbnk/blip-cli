use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct BuilderFlow {
    #[serde(rename = "application")]
    pub application: Value,
}