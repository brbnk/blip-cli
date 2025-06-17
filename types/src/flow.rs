use crate::state::State;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Flow {
    #[serde(rename = "flow")]
    pub flow: HashMap<String, State>,
}

impl Flow {
    pub fn list_state_titles(&self) {
        for (key, _) in &self.flow {
            println!("{}", key)
        }
    }

    pub fn get_onboarding_state(&self) -> Result<&State, String> {
        self.flow
            .get("onboarding")
            .ok_or("Estado 'onboarding' não encontrado".to_string())
    }

    pub fn get_state(&self, id: &String) -> Result<&State, String> {
        self.flow
            .get(id)
            .ok_or(format!("State {} não encontrado!", &id))
    }
}
