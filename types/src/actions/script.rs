use serde::{Serialize, Deserialize};
use contexts::{context};
use crate::actions::print::{print_yellow};

#[derive(Debug, Serialize, Deserialize)]
pub struct Script {
    #[serde(rename = "function")]
    pub function: String,

    #[serde(rename = "source")]
    pub source: String,

    #[serde(rename = "inputVariables")]
    pub input_variables: Vec<String>,

    #[serde(rename = "outputVariable")]
    pub output_variable: String
}

impl Script {
    pub fn execute(&self) {
        let script_response = js_runner::exec_script(self.source.clone())
            .expect("Erro ao executar script");
        
        print_yellow(
            "ExecuteScript", 
            &self.output_variable, 
            &script_response);

        context::set(&self.output_variable, &script_response);
    }
}