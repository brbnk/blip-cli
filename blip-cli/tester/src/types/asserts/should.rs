use serde::{Deserialize, Serialize};

use crate::types::asserts::Specs;

#[derive(Debug, Serialize, Deserialize)]
pub enum Should {
    BeEqual,
    BeEmpty,
    Exist,
    BeCalled,
    NotExist,
    BeSent,
    Contains
}

impl Should {
    pub fn be_equal(&self, expected: &str, observed: &str, specs: Option<&Specs>) -> Option<bool> {
        let default = expected.trim().eq(observed.trim());
        match specs {
            Some(s) => {
                if s.ignore_case {
                    Some(expected.trim().eq_ignore_ascii_case(observed.trim()))
                } else {
                    Some(default)
                }
            }
            None => Some(default),
        }
    }

    pub fn be_empty(&self, observed: &str) -> Option<bool> {
        Some(observed.is_empty())
    }

    pub fn exist(&self, expected: &str, observed: &str, specs: Option<&Specs>) -> Option<bool> {
        let default = expected.trim().eq(observed.trim());
        match specs {
            Some(s) => {
                if s.ignore_case {
                    Some(expected.trim().eq_ignore_ascii_case(observed.trim()))
                } else {
                    Some(default)
                }
            }
            None => Some(default),
        }
    }

    pub fn contains(&self, expected: &str, observed: &str, specs: Option<&Specs>) -> Option<bool> {
        let default = observed.trim().contains(expected.trim());
        match specs {
            Some(s) => {
                if s.ignore_case {
                    Some(observed.to_ascii_lowercase().trim().contains(expected.to_ascii_lowercase().trim()))
                } else {
                    Some(default)
                }
            }
            None => Some(default),
        }
    }
}
