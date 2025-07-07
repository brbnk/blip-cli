use chat::{actions::TrackEvent, custom_actions::Settings};
use serde::{Deserialize, Serialize};
use ui::printer::{self, y};
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

    pub fn assert(&self, events: &Vec<Settings>) {
        let collected_event: Option<&TrackEvent> = events
            .iter()
            .filter_map(|e| match e {
                Settings::TrackEvent(track) => Some(track),
                _ => None,
            })
            .find(|observed| {
                self.should
                    .be_equal(&self.category, &observed.category, &self.specs)
                    .unwrap_or(false)
            });

        match collected_event {
            Some(event) => match self.should {
                Should::BeEqual => match self.should.be_equal(&self.action, &event.action, &self.specs) {
                  Some(result) => printer::print_test_message(&match result {
                    true => format!("Tracking '{}' should be equal to '{}'", y(&self.category), y(&self.action)),
                    false => format!("Tracking '{}' should be equal to '{}' but '{}' was found", y(&self.category), y(&self.action), y(&event.action)) 
                  }, result),
                  None => {},
                },
                Should::BeEmpty => match self.should.be_empty(&event.action) {
                  Some(result) => printer::print_test_message(&match result {
                    true => format!("Tracking '{}' should be empty", y(&self.category)),
                    false => format!("Tracking '{}' should be empty but '{}' was found", y(&self.category), y(&event.action)) 
                  }, result),
                  None => {},
                },
                Should::Exist => match self.should.exist(&self.category, &event.category, &self.specs) {
                  Some(result) => printer::print_test_message(&match result {
                    true => format!("Tracking '{}' should exist", y(&self.category)),
                    false => format!("Tracking '{}' should exist but it was not registered", y(&self.category)) 
                  }, result),
                  None => {},
                },
                Should::NotExist => printer::print_test_message(
                  &format!("Tracking '{}' should not exist but it was registered", y(&self.category)), 
                  false),
                _ => {}
            },
            None => {
              match self.should {
                Should::NotExist => printer::print_test_message(
                  &format!("Tracking '{}' should not exist", y(&self.category)), 
                  true),
                _ => {}
              }
            },
        };
    }
}