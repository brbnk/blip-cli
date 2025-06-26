use std::collections::HashMap;

use contexts::{replacer, user};
use serde::{Deserialize, Serialize};

use crate::actions::{printer::print_cyan, Executable};

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
        match &self.name {
            Some(name) => self.save_contact_value("contact.name", &name),
            _ => {},
        }

        match &self.email {
            Some(email) => self.save_contact_value("contact.email", &email),
            _ => {},
        }
        
        match &self.city {
            Some(city) => self.save_contact_value("contact.city", &city),
            _ => {},
        }

        match &self.phone_number {
            Some(phone_number) => self.save_contact_value("contact.phoneNumber", &phone_number),
            _ => {},
        }   

        match &self.gender {
            Some(gender) => self.save_contact_value("contact.gender", &gender),
            _ => {},
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
        user::set(key, &parsed_value);
        print_cyan("MergeContact", &key.to_string(), &parsed_value);
    }
}