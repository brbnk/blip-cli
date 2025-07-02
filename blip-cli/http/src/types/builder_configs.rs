use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct BuilderConfigs {
    #[serde(rename = "configurations")]
    pub configurations: Value,
}