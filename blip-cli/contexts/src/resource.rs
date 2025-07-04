use crate::system;
use domain::constants;
use domain::traits::contexts::Manager;
use domain::traits::file_handler::Reader;
use file_handler::deserialize;
use file_handler::types::DataFile;
use once_cell::sync::Lazy;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::RwLock;

const RESOURCE_PREFIX: &str = "resource.";

pub static RESOURCES: Lazy<RwLock<HashMap<String, HashMap<String, String>>>> = Lazy::new(|| {
    let configs = HashMap::new();
    RwLock::new(configs)
});

pub struct ResourceManager {}

impl Manager for ResourceManager {
    fn get(&self, key: &str) -> Option<String> {
        let replaced_key = key.replace(RESOURCE_PREFIX, "");
        let tenant = system::get_tenant();
        let master_state = system::get_master_state();
        let pool = RESOURCES.read().unwrap();

        if let Some(master_configs) = pool.get(&master_state) {
            if let Some(value) = master_configs.get(&replaced_key) {
                return Some(value.clone());
            }
        } else {
            drop(pool);

            let resources_file = DataFile {
                tenant: tenant,
                bot_id: Some(master_state.to_string()),
                file_name: String::from(constants::RESOURCES_FILE_NAME),
                content: None,
            };

            let json = resources_file.read().expect(constants::RESOURCES_FILE_NAME);

            let resources = self.parse_resources(&json);

            self.set(&master_state, &json);

            if let Some(value) = resources.get(&replaced_key) {
                return Some(value.clone());
            }
        }
        None
    }

    fn set(&self, key: &str, value: &str) {
        let mut pool = RESOURCES.write().unwrap();
        let resources = self.parse_resources(&value.to_string());

        pool.insert(key.trim().to_string(), resources);
    }

    fn reset(&self) {
        todo!()
    }
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {}
    }

    fn parse_resources(&self, json: &String) -> HashMap<String, String> {
        let mut resources: HashMap<String, String> = HashMap::new();
    
        let deserialized_json = deserialize::<Value>(json).unwrap();
    
        if let Some(obj) = deserialized_json.as_object() {
            for (k, v) in obj {
                let key = k.trim().to_string();
                match v {
                    Value::String(s) => {
                        resources.insert(key, s.trim().to_string());
                    }
                    Value::Object(map) => {
                        resources.insert(key, serde_json::to_string(map).expect("object"));
                    }
                    _ => {
                        resources.insert(key, serde_json::to_string(v).expect("other type"));
                    }
                }
            }
        }
    
        resources
    }
}
