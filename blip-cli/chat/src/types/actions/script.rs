use contexts::{context, replacer, store};
use domain::traits::chat::Executable;
use serde::{Deserialize, Serialize};
use ui::{
    printer,
    types::{ActionProps, Color},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Script {
    #[serde(rename = "function")]
    pub function: String,

    #[serde(rename = "source")]
    pub source: String,

    #[serde(rename = "inputVariables")]
    pub input_variables: Vec<String>,

    #[serde(rename = "outputVariable")]
    pub output_variable: String,
}

impl Executable for Script {
    fn execute(&self) {
        let args: Vec<String> = self
            .input_variables
            .iter()
            .map(|input_var| match store::get(&input_var) {
                Some(value) => serde_json::to_string(&value).unwrap(),
                None => serde_json::to_string("").unwrap(),
            })
            .collect();

        let function = replacer::replace(&self.source);
        let script_response =
            js_runner::exec_script(function.clone(), args).expect("Erro ao executar script");
        context::set(&self.output_variable, &script_response);

        printer::print_action(ActionProps {
            name: String::from("ExecuteScript"),
            key: String::from(&self.output_variable),
            value: script_response,
            color: Color::Magenta,
        });
    }
}
