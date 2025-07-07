use serde::{Deserialize, Serialize};
use ui::printer;

use crate::types::asserts::Specs;

#[derive(Debug, Serialize, Deserialize)]
pub enum Should {
    BeEqual,
    BeEmpty,
    Exist,
    BeCalled,
    NotExist,
    BeSent,
}

impl Should {
    pub fn be_equal(&self, expected: &str, observed: &str, specs: &Option<Specs>) -> Option<bool> {
        match specs {
            Some(s) => {
                if s.ignore_case {
                    Some(expected.trim().eq_ignore_ascii_case(observed.trim()))
                } else {
                    Some(expected.trim().eq(observed.trim()))
                }
            }
            None => Some(expected.trim().eq(observed.trim())),
        }
    }

    pub fn be_empty(&self, observed: &str) -> Option<bool> {
        Some(observed.is_empty())
    }

    pub fn exist(&self, expected: &str, observed: &str, specs: &Option<Specs>) -> Option<bool> {
        match specs {
            Some(s) => {
                if s.ignore_case {
                    Some(expected.trim().eq_ignore_ascii_case(observed.trim()))
                } else {
                    Some(expected.trim().eq(observed.trim()))
                }
            }
            None => Some(expected.trim().eq(observed.trim())),
        }
    }

    pub fn print_result(
        &self,
        prefix: &str,
        expected: Option<&str>,
        observed: Option<&str>,
        result: Option<bool>,
    ) {
        match self {
            Should::BeEqual => match result {
                Some(r) => {
                    if r {
                        printer::print_success_message(&format!(
                            "{} '{}' should be equal to '{}'",
                            prefix,
                            expected.unwrap(),
                            observed.unwrap(),
                        ));
                    } else {
                        printer::print_error_message(&format!(
                            "{} should be equal to '{}' but '{}' was found",
                            prefix,
                            expected.unwrap(),
                            observed.unwrap()
                        ));
                    }
                }
                None => {}
            },
            Should::BeEmpty => match result {
                Some(r) => {
                    if r {
                        printer::print_success_message(&format!(
                            "{} '{}' should be empty",
                            prefix,
                            expected.unwrap()
                        ));
                    } else {
                        printer::print_error_message(&format!(
                            "{} '{}' should be empty but the value '{}' was found",
                            prefix,
                            expected.unwrap(),
                            observed.unwrap()
                        ));
                    }
                }
                None => {}
            },
            Should::Exist => match result {
                Some(r) => {
                    if r {
                        printer::print_success_message(&format!(
                            "{} '{}' should exist",
                            prefix,
                            expected.unwrap()
                        ));
                    } else {
                        printer::print_error_message(&format!(
                            "{} '{}' should exist but it was not found",
                            prefix,
                            expected.unwrap()
                        ));
                    }
                }
                None => {}
            },
            Should::BeCalled => match result {
                Some(r) => {
                    if r {
                        printer::print_success_message(&format!(
                            "{} {} should be called",
                            prefix,
                            expected.unwrap()
                        ));
                    } else {
                        printer::print_error_message(&format!(
                            "{} {} should be called but it was not",
                            prefix,
                            expected.unwrap()
                        ));
                    }
                }
                None => {}
            },
            Should::NotExist => match result {
                Some(r) => {
                    if r {
                        printer::print_success_message(&format!(
                            "{} '{}' should not exist",
                            prefix,
                            observed.unwrap()
                        ));
                    } else {
                        printer::print_error_message(&format!(
                            "{} '{}' should not exist but it was found",
                            prefix,
                            observed.unwrap()
                        ));
                    }
                }
                None => {}
            },
            Should::BeSent => match result {
                Some(r) => {
                    if r {
                        printer::print_success_message(&format!(
                            "{} {} should be sent",
                            prefix,
                            expected.unwrap()
                        ));
                    } else {
                        printer::print_error_message(&format!(
                            "{} {} should be sent but it was not",
                            prefix,
                            expected.unwrap()
                        ));
                    }
                }
                None => {}
            },
        }
    }
}
