use serde::{Deserialize, Serialize};

use super::{Should, Specs};

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessageAssert {
  #[serde(rename = "message")]
  pub message: String,

  #[serde(rename = "should")]
  pub should: Should,

  #[serde(rename = "specs")]
  pub specs: Option<Specs>,
}

impl SendMessageAssert {
  pub fn create_example() -> Self {
    Self { 
      message: "Olá, sou o bot Teste. Tudo bem com você?".to_string(), 
      should: Should::BeEqual, 
      specs: None
    }
  }
}