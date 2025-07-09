use domain::traits::{contexts::Manager, file_handler::Reader};
use file_handler::deserialize;
use file_handler::types::DataFile;
use once_cell::sync::Lazy;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::RwLock;

use crate::MANAGER_POOL;

pub static MOCKS: Lazy<RwLock<HashMap<String, String>>> = Lazy::new(|| {
    let context = HashMap::new();
    RwLock::new(context)
});

pub struct MocksManager {}

impl Manager for MocksManager {
    fn get(&self, key: &str) -> Option<String> {
        let context = MOCKS.read().unwrap();

        let value = context.get(key).cloned();

        drop(context);
        let mut writer = MOCKS.write().unwrap();
        writer.remove_entry(key);

        value
    }

    fn set(&self, key: &str, value: &str) {
        let mut context = MOCKS.write().unwrap();

        context.insert(key.trim().to_string(), value.trim().to_string());
    }

    fn reset(&self) {
        if let Ok(mut context) = MOCKS.write() {
            context.clear();
        } else {
            println!("Não foi possível resetar o contexto");
        }
    }
}

impl MocksManager {
    pub fn new() -> Self {
        MocksManager {}
    }

    pub fn init(tenant: &str, bot_id: &str, file_name: &str) {
        let file = DataFile {
            tenant: tenant.to_string(),
            bot_id: Some(bot_id.to_string()),
            file_name: file_name.to_string(),
            content: None,
        };

        let file = &file.read();
        match file {
            Ok(content) => {
                let values = deserialize::<HashMap<String, Value>>(content);
                match values {
                    Ok(v) => {
                        v.iter().for_each(|(key, value)| 
                            MANAGER_POOL.mocks.set(key,&serde_json::to_string(value).expect("serialized object")));
                    }
                    Err(_) => {}
                }
            }
            Err(_) => {}
        }
    }
}
