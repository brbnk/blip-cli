use std::collections::HashMap;
use serde::{Serialize,Deserialize};
use crate::state::State;

#[derive(Serialize,Deserialize,Debug)]
pub struct Flow {
    #[serde(rename = "flow")]
    pub flow: HashMap<String, State>
}

impl Flow {
    pub fn list_state_titles(&self) {
        for (key, _) in &self.flow {
            println!("{}", key)
        }
    }

    pub fn get_start_state(&self) {
        println!("{:#?}", &self.flow.get("onboarding"))
    }
}