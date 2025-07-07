use serde::{Deserialize, Serialize};

use crate::types::asserts::AssertType;

#[derive(Serialize, Deserialize)]
pub struct Assert {
  #[serde(rename = "asserts")]
  pub asserts: Vec<AssertType>
}