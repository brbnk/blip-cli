use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::types::asserts::{AssertType, RedirectAssert, SendMessageAssert, Specs, TrackingAssert, VariableAssert};
use file_handler::{deserialize, types::TestTemplateFile};

#[derive(Debug, Serialize, Deserialize)]
pub struct TestTemplate {
    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "inputs")]
    pub inputs: Vec<String>,

    #[serde(rename = "mocks")]
    pub mocks: HashMap<String, Value>,

    #[serde(rename = "specs")]
    pub specs: Specs,

    #[serde(rename = "asserts")]
    pub asserts: Vec<AssertType>,
}

impl TestTemplate {
    pub fn new() -> TestTemplate {
        let mut map = Map::new();

        map.insert("address".to_string(), Value::String("Rua teste".to_string()));

        TestTemplate {
            description: "Test description".to_string(),
            inputs: vec![
                String::from("olá"),
                String::from("sim")
            ],
            mocks: HashMap::from([
                ("myVar".to_string(), Value::String("my value".to_string())),
                ("apiResponse".to_string(), Value::Object(map))
            ]),
            specs: Specs::new(),
            asserts: vec![
                AssertType::Variable { inner: VariableAssert::create_example() },
                AssertType::Tracking { inner: TrackingAssert::create_example() },
                AssertType::Redirect { inner: RedirectAssert::create_example() },
                AssertType::SendMessage { inner: SendMessageAssert::create_example() }
            ],
        }
    }

    pub fn read_files(tenant: &str, identifier: &str) -> Vec<Self> {
        let instance = TestTemplateFile {
            tenant: tenant.to_string(),
            bot_id: identifier.to_string(),
            content: None,
        };

        let files = instance
            .read_files()
            .expect("test files")
            .iter()
            .map(|f| deserialize::<TestTemplate>(f).expect("test file"))
            .collect();
    
        files
    }
}
