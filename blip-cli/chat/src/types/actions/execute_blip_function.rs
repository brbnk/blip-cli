use serde::{Deserialize, Serialize};

use domain::chat::Executable;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecuteBlipFunction {
    #[serde(rename = "function")]
    pub function: String,

    #[serde(rename = "source")]
    pub source: String,

    #[serde(rename = "inputVariables")]
    pub input_variables: Vec<String>,

    #[serde(rename = "outputVariable")]
    pub output_variable: String
}

impl Executable for ExecuteBlipFunction {
    fn execute(&self) {
        todo!()
    }
}