use once_cell::sync::Lazy;
use std::collections::{HashMap};
use std::sync::RwLock;

pub static INPUT_CONTEXT: Lazy<RwLock<HashMap<String, String>>> = Lazy::new(|| {
  RwLock::new(HashMap::new())
});

pub fn get(key: &str) -> Option<String> {
  let ctx = INPUT_CONTEXT.write().unwrap();
  ctx.get(key).cloned()
}

pub fn set(key: &str, value: &str) {
  let mut ctx = INPUT_CONTEXT.write().unwrap();
  ctx.insert(key.to_string(), value.to_string());
}