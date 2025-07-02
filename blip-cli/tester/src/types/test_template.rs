use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TestTemplate {
  #[serde(rename = "description")]
  pub description: String
}

impl TestTemplate {
    pub fn new() -> TestTemplate {
        TestTemplate {
          description: "Test description".to_string()
        }
    }
}