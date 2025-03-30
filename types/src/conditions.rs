use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Condition {
    #[serde(rename = "source")]
    pub source: String,

    #[serde(rename = "comparison")]
    pub comparison: String,

    #[serde(rename = "entity")]
    pub entity: Option<String>,

    #[serde(rename = "variable")]
    pub variable: Option<String>,

    #[serde(rename = "values")]
    pub values: Vec<String>
}
