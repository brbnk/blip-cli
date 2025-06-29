use serde::{Deserialize, Serialize};

use crate::actions::Executable;

#[derive(Debug, Serialize, Deserialize)]
pub struct Redirect {
    #[serde(rename = "context")]
    pub context: RedirectContext,

    #[serde(rename = "address")]
    pub address: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedirectContext {
    #[serde(rename = "type")]
    pub redirect_type: String,

    #[serde(rename = "value")]
    pub value: String
}

impl Executable for Redirect {
    fn execute(&self) {
        todo!()
    }
}