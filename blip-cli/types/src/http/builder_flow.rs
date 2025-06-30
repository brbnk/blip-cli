use contexts::context_file_handler::handle_creation;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct BuilderFlow {
    #[serde(rename = "application")]
    pub application: Value,
}

impl BuilderFlow {
    pub fn save(&self, tenant: &str, identifier: &str) -> std::io::Result<()> {
        let content = serde_json::to_string_pretty(&self.application)?;
        
        handle_creation(
          &format!("{}/{}", tenant, identifier), 
          "flow.json", 
          &content)?;

        Ok(())
    }
}
