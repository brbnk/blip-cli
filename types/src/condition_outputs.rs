use serde::{Serialize, Deserialize};
use crate::conditions::Condition;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConditionOutputs {
  #[serde(rename = "stateId")]
  pub destination: String,

  #[serde(rename = "conditions")]
  pub conditions: Option<Vec<Condition>>
}