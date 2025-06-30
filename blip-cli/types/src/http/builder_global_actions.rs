use contexts::context_file_handler::handle_creation;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct BuilderGlobalActions {
    #[serde(rename = "globalActions")]
    pub global_actions: Value,
}

impl BuilderGlobalActions {
    pub fn save(&self, tenant: &str, identifier: &str) -> std::io::Result<()> {
        let content = serde_json::to_string_pretty(&self.global_actions)?;
        
        handle_creation(
          &format!("{}/{}", tenant, identifier),
          "global_actions.json", 
          &content)?;
          
        Ok(())
    }
}