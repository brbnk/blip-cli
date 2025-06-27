use serde::{Deserialize, Serialize};
use contexts::{context, store};
use crate::actions::{printer::{print_magenta}, Executable};

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

impl Executable for Script {
    fn execute(&self) {
        let args: Vec<String> = self.input_variables
            .iter()
            .map(|input_var| {
                match store::get(&input_var) {
                    Some(value) => serde_json::to_string(&value).unwrap(),
                    None => serde_json::to_string("").unwrap(),
                }
            })
            .collect();

        let script_response = 
            js_runner::exec_script(self.source.clone(), args)
            .expect("Erro ao executar script");
        
        print_magenta(
            "ExecuteScript", 
            &self.output_variable, 
            &script_response);

        context::set(&self.output_variable, &script_response);
    }
}