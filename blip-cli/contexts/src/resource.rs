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
  let tenant = context::get_tenant();
  let master_state = context::get_master_state();
  let pool = RESOURCES.read().unwrap();
  
  if let Some(master_configs) = pool.get(&master_state) {
    if let Some(value) = master_configs.get(&replaced_key) {
      return Some(value.clone());
    }
  }
  else {
    drop(pool);
    let path = format!("./data/{}/{}/resources.json", &tenant, master_state);
    let json = read_file_json_to_string(path).expect("Não foi possível encontrar os recursos");
    let resources = parse_resources(&json);
    set(&master_state, &resources);
    if let Some(value) = resources.get(&replaced_key) {
      return Some(value.clone());
    }
  }
  None
}

fn parse_resources(json: &String) -> HashMap<String, String> {
  let mut resources: HashMap<String, String> = HashMap::new();

  let deserialized_json = deserialize::<Value>(json).unwrap();

  if let Some(obj) = deserialized_json.as_object() {
    for (k, v) in obj {
      let key = k.trim().to_string();
      match v {
          Value::String(s) => {
            resources.insert(
              key,
              s.trim().to_string());
          },
          Value::Object(map) => {
            resources.insert(
              key,
              serde_json::to_string(map).expect("object"));
          },
          _ => {
            resources.insert(
              key,
              serde_json::to_string(v).expect("other type"));
          }
      }
    }
  }
  
  resources
}

fn set(identifier: &str, configs: &HashMap<String, String>) {
  let mut pool = RESOURCES.write().unwrap();
  pool.insert(
    identifier.trim().to_string(), 
    configs.clone());
}