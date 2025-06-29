use std::{fs::{self, File}, io::{Result, Write}};

use serde::{Deserialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct Application {
    #[serde(rename = "application")]
    pub application: Value,

    #[serde(rename = "globalActions")]
    pub global_actions: Value,

    #[serde(rename = "configurations")]
    pub configurations: Value
}

impl Application {
    fn handle_file(&self, folder_name: &str, file_name: &str, file_content: &str) -> Result<()> {
        let path = format!("./data/{}", folder_name.trim());
        fs::create_dir_all(&path).expect("created dir");

        let filename = format!("{}/{}", path, file_name);
        let mut file = File::create(filename)?;

        file.write_all(file_content.as_bytes()).expect("file created");

        Ok(())
    }

    pub fn save_flow(&self, identifier: &str) -> Result<()> {
        let content = serde_json::to_string_pretty(&self.application)?;
        self.handle_file(identifier, "flow.json", &content)?;
        Ok(())
    }

    pub fn save_global_actions(&self, identifier: &str) -> Result<()> {
        let content = &serde_json::to_string_pretty(&self.global_actions)?;
        self.handle_file(identifier, "global_actions.json", content)?;
        Ok(())
    }

    pub fn save_configurations(&self, identifier: &str) -> Result<()> {
        let content = &serde_json::to_string_pretty(&self.configurations)?;
        self.handle_file(identifier, "configs.json", content)?;
        Ok(())
    }
}