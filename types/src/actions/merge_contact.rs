use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MergeContact {
    #[serde(rename = "extras")]
    pub extras: HashMap<String, String>,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "email")]
    pub email: String,

    #[serde(rename = "city")]
    pub city: String,

    #[serde(rename = "phoneNumber")]
    pub phone_number: String,

    #[serde(rename = "taxDocument")]
    pub tax_document: String,

    #[serde(rename = "gender")]
    pub gender: String
}