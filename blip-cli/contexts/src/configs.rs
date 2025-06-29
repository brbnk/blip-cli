use once_cell::sync::Lazy;
use std::collections::{HashMap};
use std::sync::RwLock;
use json_converter::{deserialize, read_file_json_to_string};
use crate::context;

pub static CONFIGS: Lazy<RwLock<HashMap<String, HashMap<String,String>>>> = Lazy::new(|| {
  let configs = HashMap::new();
  RwLock::new(configs)
});

pub fn get(key: &str) -> Option<String> {
  let replaced_key = key.replace("config.", "");
  let master_state = context::get_master_state();
  let pool = CONFIGS.read().unwrap();
  
  if let Some(master_configs) = pool.get(&master_state) {
    if let Some(value) = master_configs.get(&replaced_key) {
      return Some(value.clone());
    }
  }
  else {
    drop(pool);
    let path = format!("./data/{}/configs.json", master_state);
    let json = read_file_json_to_string(path).expect("Não foi possível encontrar as ações globais");
    let configs = deserialize::<HashMap<String, String>>(&json).unwrap();
    set(&master_state, &configs);
    if let Some(value) = configs.get(&replaced_key) {
      return Some(value.clone());
    }
  }
  None
}

pub fn set(identifier: &str, configs: &HashMap<String, String>) {
  let mut pool = CONFIGS.write().unwrap();
  pool.insert(
    identifier.trim().to_string(), 
    configs.clone());
}