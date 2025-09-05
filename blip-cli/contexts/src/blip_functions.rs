use crate::system;
use domain::constants;
use domain::contexts::Manager;
use domain::file_handler::Reader;
use file_handler::deserialize;
use file_handler::types::DataFile;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::RwLock;

const BLIP_FUNCTIONS_PREFIX: &str = "blipfunction.";

pub static BLIP_FUNCTIONS: Lazy<RwLock<HashMap<String, HashMap<String, String>>>> = Lazy::new(|| {
    let blip_functions = HashMap::new();
    RwLock::new(blip_functions)
});

pub struct BlipFunctionsManager {}

impl BlipFunctionsManager {
    pub fn new() -> Self {
        BlipFunctionsManager {}
    }
}

impl Manager for BlipFunctionsManager {
    fn get(&self, key: &str) -> Option<String> {
        let replaced_key = key.replace(BLIP_FUNCTIONS_PREFIX, "");
        let tenant = system::get_tenant();
        
        let pool = BLIP_FUNCTIONS.read().unwrap();

        if let Some(functions) = pool.get(&tenant) {
            if let Some(value) = functions.get(&replaced_key) {
                return Some(value.clone());
            }
        } else {
            drop(pool);

            let blip_functions_file = DataFile {
                tenant: tenant.clone(),
                bot_id: None,
                file_name: String::from(constants::BLIP_FUNCTION_FILE_NAME),
                content: None,
            };

            let json = blip_functions_file.read().expect(constants::BLIP_FUNCTION_FILE_NAME);
            
            let functions: HashMap<String, String> = 
                deserialize::<HashMap<String, String>>(&json).expect("deserialized blip functions");
            
            self.set(&tenant, &serde_json::to_string(&functions).expect("serialized blip functions"));

            let value = functions.get(&replaced_key);
            if value.is_some() {
                return value.cloned();
            }
        }
        None
    }

    fn set(&self, key: &str, value: &str) {
        let mut pool = BLIP_FUNCTIONS.write().unwrap();
        
        let functions = 
            deserialize::<HashMap<String, String>>(&value.to_string()).expect("deserialized blip functions");

        pool.insert(
            key.trim().to_string(), 
            functions);
    }

    fn delete(&self, key: &str) {
        let mut writer = BLIP_FUNCTIONS.write().unwrap();
        writer.remove_entry(key);
    }

    fn reset(&self) {
        todo!()
    }
}
