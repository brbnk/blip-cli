use crate::system;
use domain::constants;
use domain::contexts::Manager;
use domain::file_handler::Reader;
use file_handler::deserialize;
use file_handler::types::DataFile;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::RwLock;

use serde::{Deserialize, Serialize};

const BLIP_FUNCTION_PREFIX: &str = "blipfunction.";

pub static BLIP_FUNCTIONS: Lazy<RwLock<HashMap<String, String>>> = Lazy::new(|| {
    let blip_functions = HashMap::new();
    RwLock::new(blip_functions)
});

pub struct BlipFunctionsManager {}

impl BlipFunctionsManager {
    pub fn new() -> Self {
        BlipFunctionsManager {}
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlipFunctionSchema {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "content")]
    pub content: String
}

impl Manager for BlipFunctionsManager {
    fn get(&self, key: &str) -> Option<String> {
        let tenant = system::get_tenant();
        
        let pool = BLIP_FUNCTIONS.read().unwrap();

        if let Some(fs) = pool.get(&tenant) {
            let functions: Vec<BlipFunctionSchema> = deserialize::<Vec<BlipFunctionSchema>>(&fs).expect("deserialized blip functions");
            let bfs = functions
                .iter()
                .find(|f| { 
                    if !key.contains(BLIP_FUNCTION_PREFIX) {
                        f.id.eq(&key)
                    }
                    else {
                        let function_name = key.replace(BLIP_FUNCTION_PREFIX, "");
                        f.name.eq(&function_name)
                    }
                });

            if bfs.is_some() {
                return Some(bfs.unwrap().content.clone());
            }
        } else {
            drop(pool);

            let blip_functions_file = DataFile {
                tenant: tenant.clone(),
                bot_id: None,
                file_name: String::from(constants::BLIP_FUNCTION_FILE_NAME),
                content: None,
            };

            match blip_functions_file.read() {
                Ok(content) => {
                    let functions: Vec<BlipFunctionSchema> = deserialize::<Vec<BlipFunctionSchema>>(&content).expect("deserialized blip functions");
                    self.set(&tenant, &serde_json::to_string(&functions).expect("serialized blip functions"));
                    
                    let value = functions
                        .iter()
                        .find(|f| { 
                            if !key.contains(BLIP_FUNCTION_PREFIX) {
                                f.id.eq(&key)
                            }
                            else {
                                let function_name = key.replace(BLIP_FUNCTION_PREFIX, "");
                                f.name.eq(&function_name)
                            }
                        });
                    
                    if value.is_some() {
                        return Some(value.unwrap().content.clone());
                    }
                },
                Err(_) => {
                    self.set(&tenant, "{}");
                },
            }
            
        }
        None
    }

    fn set(&self, key: &str, value: &str) {
        let mut pool = BLIP_FUNCTIONS.write().unwrap();
        
        let functions = 
            deserialize::<Vec<BlipFunctionSchema>>(&value.to_string()).expect("deserialized blip functions");

        pool.insert(
            key.trim().to_string(), 
            serde_json::to_string(&functions).expect("serialized tenant functions"));
    }

    fn delete(&self, key: &str) {
        let mut writer = BLIP_FUNCTIONS.write().unwrap();
        writer.remove_entry(key);
    }

    fn reset(&self) {
        todo!()
    }
}
