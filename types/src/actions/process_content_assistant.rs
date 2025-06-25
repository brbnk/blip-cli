use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessContentAssistant {
    #[serde(rename = "v2")]
    pub v2: bool,

    #[serde(rename = "text")]
    pub text: String,

    #[serde(rename = "score")]
    pub score: String,

    #[serde(rename = "outputVariable")]
    pub output_variable: String
}