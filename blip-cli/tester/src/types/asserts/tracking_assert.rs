use serde::{Deserialize, Serialize};

use super::{Should, Specs};

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackingAssert {
  #[serde(rename = "category")]
  pub category: String,

  #[serde(rename = "should")]
  pub should: Should,

  #[serde(rename = "action")]
  pub action: String,

  #[serde(rename = "specs")]
  pub specs: Option<Specs>,
}

impl TrackingAssert {
  pub fn create_example() -> Self {
    Self { 
      category: "Category tracking examplo".to_string(), 
      should: Should::BeEqual, 
      action: "Value tracking".to_string(), 
      specs: None
    }
  }
}