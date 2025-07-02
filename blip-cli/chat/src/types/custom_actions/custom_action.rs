use serde::{Serialize, Deserialize};
use super::{ActionType, Settings};
use crate::types::{execute_conditions::Condition};

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomAction {
    #[serde(rename = "$id")]
    pub id: String,

    #[serde(rename = "type")]
    pub action_type: ActionType,

    #[serde(rename = "$title")]
    pub title: String,

    #[serde(rename = "settings")]
    pub settings: Settings,

    #[serde(rename = "conditions")]
    pub conditions: Option<Vec<Condition>>
}

impl CustomAction {
    pub fn should_execute(&self) -> bool {
        match &self.conditions {
            Some(conditions) => {
                if conditions.is_empty() {
                    return true
                }

                return conditions
                    .iter()
                    .any(|c| c.should_execute())
            },
            None => true,
        }
    }

    pub fn execute(&self) {
        self.settings.as_executable().execute();
    }
}