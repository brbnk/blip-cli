use std::collections::HashMap;

use contexts::{replacer, system, MANAGER_POOL};
use serde::{Deserialize, Serialize};

use domain::traits::{chat::Executable};
use ui::{printer, types::{ActionProps, Color}};

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
    pub gender: Option<String>,
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

        MANAGER_POOL.context.set(key, &parsed_value);

        if !system::is_test_mode() {
            printer::print_action(ActionProps {
                name: String::from("MergeContact"),
                key: String::from(replacer::replace(key)),
                value: parsed_value,
                color: Color::Cyan
            });
        }
    }
}
