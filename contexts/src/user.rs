use once_cell::sync::Lazy;
use std::collections::{HashMap};
use std::sync::RwLock;

pub static USER_CONTEXT: Lazy<RwLock<HashMap<String, String>>> = Lazy::new(|| {
  let mut context = HashMap::new();

  context.insert(String::from("contact.name"), String::from("Bruno"));

  RwLock::new(context)
});

pub fn get(key: &str) -> Option<String> {
  let ctx = USER_CONTEXT.write().unwrap();
  ctx.get(key).cloned()
}

pub fn set(key: &str, value: &str) {
  let mut ctx = USER_CONTEXT.write().unwrap();
  ctx.insert(key.to_string(), value.to_string());
}