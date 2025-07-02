use serde::{Deserialize, Serialize};
use super::{Action, Input};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContentAction {
    Action { action: Action },
    Input { input: Input },
}