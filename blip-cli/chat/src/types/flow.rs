use super::State;
use contexts::{MANAGER_POOL};
use file_handler::deserialize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Flow {
    #[serde(rename = "flow")]
    pub flow: HashMap<String, State>,
}

impl Flow {
    pub fn deserialize(identifier: &str) -> Flow {
        let serialized_flow = MANAGER_POOL.flow.get(identifier);
        let flow: Flow = match serialized_flow {
            Some(flow) => deserialize(&flow).expect("Falha ao desserializar o parse do fluxo!"),
            None => panic!(),
        };
        flow
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
