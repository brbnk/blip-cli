use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct BuilderGlobalActions {
    #[serde(rename = "globalActions")]
    pub global_actions: Value,
}
