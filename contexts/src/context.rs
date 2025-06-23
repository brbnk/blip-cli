use once_cell::sync::Lazy;
use std::collections::{HashMap};
use std::sync::RwLock;

pub static CONTEXT: Lazy<RwLock<HashMap<String, String>>> = Lazy::new(|| {
  let context = HashMap::new();
  RwLock::new(context)
});

pub fn get(key: &str) -> Option<String> {
  let ctx = CONTEXT.write().unwrap();
  ctx.get(key).cloned()
}

pub fn set(key: &str, value: &str) {
  let mut ctx = CONTEXT.write().unwrap();
  ctx.insert(key.to_string(), value.to_string());
}