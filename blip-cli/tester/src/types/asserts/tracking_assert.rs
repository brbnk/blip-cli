use chat::{actions::TrackEvent, custom_actions::Settings};
use serde::{Deserialize, Serialize};
use ui::printer::{self, y};
use crate::types::TestTemplate;

use super::{Should, Specs};

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackingAssert {
    #[serde(rename = "category")]
    pub category: String,

    #[serde(rename = "should")]
    pub should: Should,

    #[serde(rename = "action")]
    pub action: String,

    #[serde(rename = "specs")]
    pub specs: Option<Specs>,
}

impl TrackingAssert {
    pub fn create_example() -> Self {
        Self {
            category: "Category tracking examplo".to_string(),
            should: Should::BeEqual,
            action: "Value tracking".to_string(),
            specs: None,
        }
    }

    pub fn assert(&self, events: &Vec<Settings>, template: &TestTemplate) {
        let category = &self.category;
        let action = &self.action;
        let specs = match &self.specs {
          Some(s) => Some(s),
          None => Some(&template.specs),
        };

        let collected_event: Option<&TrackEvent> = events
            .iter()
            .filter_map(|e| match e {
                Settings::TrackEvent(track) => Some(track),
                _ => None,
            })
            .find(|observed| {
                self.should
                    .be_equal(&category, &observed.category, specs)
                    .unwrap_or(false)
            });

        match collected_event {
            Some(event) => match self.should {
                Should::BeEqual => match self.should.be_equal(&action, &event.action, specs) {
                  Some(result) => printer::print_test_message(&match result {
                    true => format!("Tracking '{}' should be equal to '{}'", y(&category), y(&action)),
                    false => format!("Tracking '{}' should be equal to '{}' but '{}' was found", y(&category), y(&action), y(&event.action)) 
                  }, result),
                  None => {},
                },
                Should::BeEmpty => match self.should.be_empty(&event.action) {
                  Some(result) => printer::print_test_message(&match result {
                    true => format!("Tracking '{}' should be empty", y(&category)),
                    false => format!("Tracking '{}' should be empty but '{}' was found", y(&category), y(&event.action)) 
                  }, result),
                  None => {},
                },
                Should::Exist => match self.should.exist(&category, &event.category, specs) {
                  Some(result) => printer::print_test_message(&match result {
                    true => format!("Tracking '{}' should exist", y(&category)),
                    false => format!("Tracking '{}' should exist but it was not registered", y(&category)) 
                  }, result),
                  None => {},
                },
                Should::Contains => match  self.should.contains(&action, &event.action, specs) {
                  Some(result) => printer::print_test_message(&match result {
                    true => format!("Tracking '{}' should contain '{}' in its action value", y(&category), y(&action)),
                    false => format!("Tracking '{}' should contain '{}' in its action value, but it was not found on '{}'", y(&category), y(&action), y(&event.action)) 
                  }, result),
                  None => {},
                },
                Should::NotExist => printer::print_test_message(
                  &format!("Tracking '{}' should not exist but it was registered", y(&category)), 
                  false),
                _ => {}
            },
            None => {
              match self.should {
                Should::NotExist => printer::print_test_message(
                  &format!("Tracking '{}' should not exist", y(&category)), 
                  true),
                _ => {}
              }
            },
        };
    }
}