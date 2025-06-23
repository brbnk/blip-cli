use serde::{Serialize, Deserialize};
use crate::conditions::Condition;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConditionOutputs {
  #[serde(rename = "stateId")]
  pub destination: String,

  #[serde(rename = "conditions")]
  pub conditions: Option<Vec<Condition>>
}

impl ConditionOutputs {
  pub fn get_destination(&self) -> Option<&String> {
    match &self.conditions {
      Some(conditions) => {
        for condition in conditions {
          if condition.should_execute() {
            return Some(&self.destination)
          }
        }
        return None
      },
      None => None
    }
  }
}