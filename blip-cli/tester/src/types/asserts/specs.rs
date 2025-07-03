use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Specs {
  #[serde(rename = "ignoreCase")]
  pub ignore_case: bool
}

impl Specs {
  pub fn new() -> Self {
    Self { 
      ignore_case: true
    }
  }
}