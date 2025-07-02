use serde::{Deserialize, Serialize};

use domain::traits::chat::Executable;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessCommand {
    #[serde(rename = "to")]
    pub to: String,

    #[serde(rename = "method")]
    pub method: String,

    #[serde(rename = "uri")]
    pub uri: String,

    #[serde(rename = "variable")]
    pub variable: String
}

impl Executable for ProcessCommand {
    fn execute(&self) {
        todo!()
    }
}