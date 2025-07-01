use contexts::context_file_handler;
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

    pub fn create_file(tenant: &str, bot: &str) {
      let template: TestTemplate = TestTemplate::new();
      let content = serde_json::to_string_pretty(&template).expect("template file");
      
      context_file_handler::handle_test_file_creation(
        &format!("{}/{}", tenant, bot), 
        &content
      ).expect("test file created");
    }
}