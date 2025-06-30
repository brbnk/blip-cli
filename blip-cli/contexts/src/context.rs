use once_cell::sync::Lazy;
use std::collections::{HashMap};
use std::sync::RwLock;

pub static CONTEXT: Lazy<RwLock<HashMap<String, String>>> = Lazy::new(|| {
  let context = HashMap::new();
  RwLock::new(context)
});

pub fn get(key: &str) -> Option<String> {
  let ctx = CONTEXT.read().unwrap();
  ctx.get(key).cloned()
}

pub fn set(key: &str, value: &str) {
  let mut ctx = CONTEXT.write().unwrap();
  ctx.insert(key.trim().to_string(), value.trim().to_string());
}

pub fn get_master_state() -> String {
  let result = get("master-state");
  match result {
    Some(master_state) => master_state,
    None => panic!("master-state não encontrado"),
  }
}

pub fn set_master_state(value: &str) {
  set("master-state", value);
}

pub fn get_tenant() -> String {
  let result = get("tenant");
  match result {
    Some(tenant) => tenant,
    None => panic!("tenant não encontrado"),
  }
}

pub fn set_tenant(value: &str) {
  set("tenant", value);
}