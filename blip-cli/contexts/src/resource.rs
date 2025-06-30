use once_cell::sync::Lazy;
use serde_json::{Value};
use std::collections::{HashMap};
use std::sync::RwLock;
use json_converter::{deserialize, read_file_json_to_string};
use crate::context;

pub static RESOURCES: Lazy<RwLock<HashMap<String, HashMap<String,String>>>> = Lazy::new(|| {
  let configs = HashMap::new();
  RwLock::new(configs)
});

pub fn get(key: &str) -> Option<String> {
  let replaced_key = key.replace("resource.", "");
  let master_state = context::get_master_state();
  let pool = RESOURCES.read().unwrap();
  
  if let Some(master_configs) = pool.get(&master_state) {
    if let Some(value) = master_configs.get(&replaced_key) {
      return Some(value.clone());
    }
  }
  else {
    drop(pool);
    let path = format!("./data/{}/resources.json", master_state);
    let json = read_file_json_to_string(path).expect("Não foi possível encontrar os recursos");
    let deserialized_json = deserialize::<Value>(&json).unwrap();

    let mut resources: HashMap<String, String> = HashMap::new();
    if let Some(obj) = deserialized_json.as_object() {
      for (k, v) in obj {
        let key = k.trim();
        match v {
            Value::String(s) => {
              resources.insert(key.to_string(), s.trim().to_string());
            },
            Value::Object(map) => {
              resources.insert(key.to_string(), serde_json::to_string(map).expect("object"));
            },
            _ => {
              resources.insert(key.to_string(), serde_json::to_string(v).expect("other type"));
            }
        }
      }
    }
    
    set(&master_state, &resources);
    if let Some(value) = resources.get(&replaced_key) {
      return Some(value.clone());
    }
  }
  None
}

fn set(identifier: &str, configs: &HashMap<String, String>) {
  let mut pool = RESOURCES.write().unwrap();
  pool.insert(
    identifier.trim().to_string(), 
    configs.clone());
}