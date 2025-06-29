use std::collections::HashMap;

use contexts::{replacer, contact};
use serde::{Deserialize, Serialize};

use crate::actions::{Executable};

#[derive(Debug, Serialize, Deserialize)]
pub struct MergeContact {
    #[serde(rename = "extras")]
    pub extras: HashMap<String, String>,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "email")]
    pub email: Option<String>,

    #[serde(rename = "city")]
    pub city: Option<String>,

    #[serde(rename = "phoneNumber")]
    pub phone_number: Option<String>,

    #[serde(rename = "taxDocument")]
    pub tax_document: Option<String>,

    #[serde(rename = "gender")]
    pub gender: Option<String>
}

impl Executable for MergeContact {
    fn execute(&self) {
        if self.name.is_some() {
            self.save_contact_value("contact.name", self.name.as_ref().unwrap())
        }

        if self.email.is_some() {
            self.save_contact_value("contact.email", self.email.as_ref().unwrap())
        }

        if self.city.is_some() {
            self.save_contact_value("contact.city", self.city.as_ref().unwrap())
        }

        if self.phone_number.is_some() {
            self.save_contact_value("contact.phoneNumber", self.phone_number.as_ref().unwrap())
        }

        if self.gender.is_some() {
            self.save_contact_value("contact.gender", self.gender.as_ref().unwrap())
        }

        for (key, value) in &self.extras {
            if !value.is_empty() {
                self.save_contact_value(&format!("contact.extras.{}", key), &value)
            }
        }
    }
}

impl MergeContact {
    fn save_contact_value(&self, key: &str, value: &str) {
        let parsed_value = replacer::replace(value);
        contact::set(key, &parsed_value);
        ui::printer::print_cyan("MergeContact", &key.to_string(), &parsed_value);
    }
}