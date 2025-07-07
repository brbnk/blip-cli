use chat::{actions::Script, custom_actions::Settings};
use contexts::MANAGER_POOL;
use serde::{Deserialize, Serialize};
use ui::printer::{self, y};

use crate::types::TestTemplate;

use super::{Should, Specs};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecuteScriptAssert {
    #[serde(rename = "outputVar")]
    pub output_variable: String,

    #[serde(rename = "should")]
    pub should: Should,

    #[serde(rename = "value")]
    pub value: Option<String>,

    #[serde(rename = "specs")]
    pub specs: Option<Specs>,
}

impl ExecuteScriptAssert {
    pub fn create_example() -> Self {
        Self {
            output_variable: "exampleVar".to_string(),
            should: Should::BeEqual,
            value: Some("value".to_string()),
            specs: None,
        }
    }

    pub fn assert(&self, events: &Vec<Settings>, template: &TestTemplate) {
        let variable = &self.output_variable;
        let expected = self.value.clone().unwrap_or("".to_owned());
        let specs: Option<&Specs> = match &self.specs {
            Some(s) => Some(s),
            None => Some(&template.specs),
        };

        let collected_event: Option<&Script> = events
            .iter()
            .filter_map(|e| match e {
                Settings::Script(s) => Some(s),
                _ => None,
            })
            .find(|observed| {
                self.should
                    .be_equal(variable, &observed.output_variable, specs)
                    .unwrap_or(false)
            });

        match collected_event {
            Some(event) => {
                let observed = MANAGER_POOL.context
                    .get(&event.output_variable)
                    .ok_or("".to_owned())
                    .expect("script output response");

                match self.should {
                    Should::BeEqual => match self.should.be_equal(&expected, &observed, specs) {
                      Some(result) => printer::print_test_message(&match result {
                        true => format!("Script '{}' response should be equal to '{}'", y(&variable), y(&expected)),
                        false => format!("Script '{}' response should be equal to '{}' but '{}' was found", y(&variable), y(&expected), y(&observed)) 
                      }, result),
                      None => {},
                    },
                    Should::BeEmpty => match self.should.be_empty(&observed) {
                      Some(result) => printer::print_test_message(&match result {
                        true => format!("Script '{}' response should be empty", y(&variable)),
                        false => format!("Script '{}' response should be empty but '{}' value was found", y(&variable), y(&observed)) 
                      }, result),
                      None => {},
                    },
                    Should::Exist => match self.should.exist(&expected, &observed, specs) {
                      Some(result) => printer::print_test_message(&match result {
                        true => format!("Script '{}' output variable should exist", y(&variable)),
                        false => format!("Script '{}' ouput variable should exist but it was not found on context", y(&variable)) 
                      }, result),
                      None => {},
                    },
                    Should::Contains => match  self.should.contains(&expected, &observed, specs) {
                      Some(result) => printer::print_test_message(&match result {
                        true => format!("Script '{}' response should contain {}", y(&variable), y(&expected)),
                        false => format!("Script '{}' response should contain {} but it was not found on {}", y(&variable), y(&expected), y(&observed)) 
                      }, result),
                      None => {},
                    },
                    Should::NotExist => printer::print_test_message(
                      &format!("Script '{}' output variable should not exist but it was found on context", y(&variable)), 
                      false),
                    _ => {}
                }
            }
            None => {
              match self.should {
                Should::NotExist => printer::print_test_message(
                  &format!("Script '{}' ouput variable should not exist", y(&variable)), 
                  true),
                _ => printer::print_test_message(
                    &format!("Script '{}' output variable was not registered during test", y(&variable)), 
                    false)
              }
            },
        };
    }
}
