use serde::{Deserialize, Serialize};

use super::{Should, Specs};

#[derive(Debug, Serialize, Deserialize)]
pub struct VariableAssert {
  #[serde(rename = "variable")]
  pub variable: String,

  #[serde(rename = "should")]
  pub should: Should,

  #[serde(rename = "value")]
  pub value: Option<String>,

  #[serde(rename = "specs")]
  pub specs: Option<Specs>,
}

impl VariableAssert {
  pub fn create_example() -> Self {
    Self { 
      variable: "exampleVar".to_string(), 
      should: Should::BeEqual, 
      value: Some("value".to_string()), 
      specs: None
    }
  }
}