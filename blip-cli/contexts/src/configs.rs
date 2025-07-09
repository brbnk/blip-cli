use crate::{system};
use file_handler::{types::DataFile, deserialize};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::RwLock;
use domain::{constants, {file_handler::Reader, contexts::Manager}};

const CONFIG_VAR_PREFIX: &str = "config.";

pub static CONFIGS: Lazy<RwLock<HashMap<String, HashMap<String, String>>>> = Lazy::new(|| {
    let configs = HashMap::new();
    RwLock::new(configs)
});

pub struct ConfigManager {}

impl ConfigManager {
    pub fn new() -> Self {
        ConfigManager {}
    }
}

impl Manager for ConfigManager {
    fn get(&self, key: &str) -> Option<String> {
        let replaced_key = key.replace(CONFIG_VAR_PREFIX, "");
        let master_state = system::get_master_state();
        let tenant = system::get_tenant();
        let pool = CONFIGS.read().unwrap();
    
        if let Some(master_configs) = pool.get(&master_state) {
            if let Some(value) = master_configs.get(&replaced_key) {
                return Some(value.clone());
            }
        } else {
            drop(pool);
            let data_file = DataFile {
                tenant: tenant,
                bot_id: Some(master_state.to_string()),
                file_name: String::from(constants::CONFIGS_FILE_NAME),
                content: None,
            };
    
            let json = data_file
                .read()
                .expect(constants::CONFIGS_FILE_NAME);

            let configs: HashMap<String, String> = 
                deserialize::<HashMap<String, String>>(&json).expect("deserialized configutarions");
            
            self.set(&master_state, &serde_json::to_string(&configs).expect("serialized configs"));

            let value = configs.get(&replaced_key);
            if value.is_some() {
                return value.cloned();
            }
        }
        None
    }
    
    fn set(&self, key: &str, value: &str) {
        let mut pool = CONFIGS.write().unwrap();
        
        let configs = 
            deserialize::<HashMap<String, String>>(&value.to_string()).expect("deserialized configutarions");

        pool.insert(
            key.trim().to_string(), 
            configs);
    }
    
    fn reset(&self) {
        todo!()
    }
}
