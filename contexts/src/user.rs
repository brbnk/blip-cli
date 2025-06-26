use once_cell::sync::Lazy;
use std::collections::{HashMap};
use std::sync::RwLock;

pub static USER_CONTEXT: Lazy<RwLock<HashMap<String, String>>> = Lazy::new(|| {
  RwLock::new(HashMap::new())
});

pub fn get(key: &str) -> Option<String> {
  let ctx = USER_CONTEXT.read().unwrap();
  ctx.get(key).cloned()
}

pub fn set(key: &str, value: &str) {
  let mut ctx = USER_CONTEXT.write().unwrap();
  ctx.insert(key.trim().to_string(), value.trim().to_string());
}