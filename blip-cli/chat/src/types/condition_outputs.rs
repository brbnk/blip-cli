use super::execute_conditions::Condition;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConditionOutputs {
    #[serde(rename = "stateId")]
    pub destination: String,

    #[serde(rename = "conditions")]
    pub conditions: Option<Vec<Condition>>,
}

impl ConditionOutputs {
    pub fn get_destination(&self) -> Option<&String> {
        match &self.conditions {
            Some(conditions) => {
                let all_conditions_satisfied = conditions
                    .iter()
                    .all(|c| c.should_execute());

                if !all_conditions_satisfied {
                    return None;
                }

                return Some(&self.destination);
            }
            None => None,
        }
    }
}
