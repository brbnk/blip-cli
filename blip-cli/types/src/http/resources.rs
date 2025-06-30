use contexts::context_file_handler::handle_creation;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct Resources {
    #[serde(rename = "resources")]
    pub resources: Value,
}

impl Resources {
    pub fn save(&self, identifier: &str) -> std::io::Result<()> {
        let content = serde_json::to_string_pretty(&self.resources)?;
        
        handle_creation(
          identifier, 
          "resources.json", 
          &content)?;
          
        Ok(())
    }
}