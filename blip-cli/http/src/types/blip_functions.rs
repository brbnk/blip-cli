use contexts::context_file_handler::handle_creation;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct BlipFunctionsResult {
    #[serde(rename = "tenant")]
    pub tenant: String,

    #[serde(rename = "functions")]
    pub functions: Vec<Value>,
}

impl BlipFunctionsResult {
    pub fn save(&self) -> std::io::Result<()> {
        let content = serde_json::to_string_pretty(&self.functions)?;

        handle_creation(
            &format!("{}", &self.tenant),
            "blip-functions.json",
            &content,
        )?;

        Ok(())
    }
}
