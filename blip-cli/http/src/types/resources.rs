use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct Resources {
    #[serde(rename = "resources")]
    pub resources: Value,
}