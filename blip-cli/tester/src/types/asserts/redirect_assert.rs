use serde::{Deserialize, Serialize};

use super::{Should, Specs};

#[derive(Debug, Serialize, Deserialize)]
pub struct RedirectAssert {
  #[serde(rename = "service")]
  pub service: String,

  #[serde(rename = "should")]
  pub should: Should,

  #[serde(rename = "withContextMessage")]
  pub with_context_message: Option<String>,

  #[serde(rename = "specs")]
  pub specs: Option<Specs>,
}

impl RedirectAssert {
  pub fn create_example() -> Self {
    Self { 
      service: "main".to_string(), 
      should: Should::BeCalled, 
      with_context_message: Some("redirect from child".to_string()),
      specs: None
    }
  }
}