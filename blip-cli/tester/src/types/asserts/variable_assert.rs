use chat::{actions::Variable, custom_actions::Settings};
use serde::{Deserialize, Serialize};
use ui::printer::{self, y};

use super::{Should, Specs};

#[derive(Debug, Serialize, Deserialize)]
pub struct VariableAssert {
    #[serde(rename = "variable")]
    pub variable: String,

    #[serde(rename = "should")]
    pub should: Should,

    #[serde(rename = "value")]
    pub value: Option<String>,

    #[serde(rename = "specs")]
    pub specs: Option<Specs>,
}

impl VariableAssert {
    pub fn create_example() -> Self {
        Self {
            variable: "exampleVar".to_string(),
            should: Should::BeEqual,
            value: Some("value".to_string()),
            specs: None,
        }
    }

    pub fn assert(&self, events: &Vec<Settings>) {
        let variable = &self.variable;
        let exepected = self.value.clone().unwrap_or("".to_owned());

        let collected_event: Option<&Variable> = events
            .iter()
            .filter_map(|e| match e {
                Settings::Variable(v) => Some(v),
                _ => None,
            })
            .find(|observed| {
                self.should
                    .be_equal(&self.variable, &observed.variable, &self.specs)
                    .unwrap_or(false)
            });

        match collected_event {
            Some(event) => {
                let observed = event.value.clone().unwrap_or("".to_owned());
                match self.should {
                    Should::BeEqual => match self.should.be_equal(&exepected, &observed, &self.specs) {
                      Some(result) => printer::print_test_message(&match result {
                        true => format!("Variable '{}' should be equal to '{}'", y(&variable), y(&exepected)),
                        false => format!("Variable '{}' should be equal to '{}' but '{}' was found", y(&variable), y(&exepected), y(&observed)) 
                      }, result),
                      None => {},
                    },
                    Should::BeEmpty => match self.should.be_empty(&observed) {
                      Some(result) => printer::print_test_message(&match result {
                        true => format!("Variable '{}' should be empty", y(&variable)),
                        false => format!("Variable '{}' should be empty but '{}' value was found", y(&variable), y(&observed)) 
                      }, result),
                      None => {},
                    },
                    Should::Exist => match self.should.exist(&exepected, &observed, &self.specs) {
                      Some(result) => printer::print_test_message(&match result {
                        true => format!("Variable '{}' should exist", y(&variable)),
                        false => format!("Variable '{}' should exist but it was not registered", y(&variable)) 
                      }, result),
                      None => {},
                    },
                    Should::NotExist => printer::print_test_message(
                      &format!("Varible '{}' should not exist but it was registered", y(&variable)), 
                      false),
                    _ => {}
                }
            }
            None => {
              match self.should {
                Should::NotExist => printer::print_test_message(
                  &format!("Variable '{}' should not exist", y(&variable)), 
                  true),
                _ => printer::print_test_message(
                    &format!("Variable '{}' was not registered during test", y(&variable)), 
                    false)
              }
            },
        };
    }
}
