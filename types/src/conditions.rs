use serde::{Serialize, Deserialize};
use contexts::{input,context};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Source {
    Input,
    Context,
    Entity,
    Intent
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Comparison {
    Exists,
    Equals,
    NotEquals,
    Contains,
    StartsWith,
    EndsWith,
    GreaterThan,
    LessThan,
    GreaterThanOrEquals,
    LessThanOrEquals,
    ApproximateTo,
    Matches,
    NotExists
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Condition {
    #[serde(rename = "source")]
    pub source: Source,

    #[serde(rename = "comparison")]
    pub comparison: Comparison,

    #[serde(rename = "entity")]
    pub entity: Option<String>,

    #[serde(rename = "variable")]
    pub variable: Option<String>,

    #[serde(rename = "values")]
    pub values: Vec<String>
}

impl Condition {
    pub fn should_execute(&self) -> bool {
        self.compare()
    }

    fn compare(&self) -> bool {
        let value = self.get_source_value();
        match self.comparison {
            Comparison::Exists => {
                match value {
                    Some(v) => v != "",
                    None => false,
                }
            },
            Comparison::Equals => {
                match value {
                    Some(v) => self.values
                        .iter()
                        .any(|s| s.eq_ignore_ascii_case(v.as_str())),
                    None => false,
                }
            },
            Comparison::NotEquals => todo!(),
            Comparison::Contains => todo!(),
            Comparison::StartsWith => todo!(),
            Comparison::EndsWith => todo!(),
            Comparison::GreaterThan => todo!(),
            Comparison::LessThan => todo!(),
            Comparison::GreaterThanOrEquals => todo!(),
            Comparison::LessThanOrEquals => todo!(),
            Comparison::ApproximateTo => todo!(),
            Comparison::Matches => todo!(),
            Comparison::NotExists => todo!(),
        }
    }

    fn get_source_value(&self) -> Option<String> {
        match self.source {
            Source::Input => 
                input::get("input.content"),
            Source::Context => 
                context::get(self.variable
                    .as_ref()
                    .unwrap()
                    .as_str()),
            Source::Entity => todo!(),
            Source::Intent => todo!(),
        }
    }
}