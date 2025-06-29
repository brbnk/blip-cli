use contexts::context_file_handler::handle_creation;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct BuilderConfigs {
    #[serde(rename = "configurations")]
    pub configurations: Value,
}

impl BuilderConfigs {
    pub fn save(&self, identifier: &str) -> std::io::Result<()> {
        let content = serde_json::to_string_pretty(&self.configurations)?;
        
        handle_creation(
          identifier, 
          "configs.json", 
          &content)?;
          
        Ok(())
    }
}