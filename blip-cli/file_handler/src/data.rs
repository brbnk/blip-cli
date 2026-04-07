use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RouterChild {
    #[serde(rename = "shortName")]
    pub short_name: String
}