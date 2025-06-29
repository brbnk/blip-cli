use once_cell::sync::Lazy;
use std::collections::{HashMap};
use json_converter::read_file_json_to_string;
use std::sync::RwLock;

pub static FLOWS: Lazy<RwLock<HashMap<String, String>>> = Lazy::new(|| {
  RwLock::new(HashMap::new())
});

pub fn get(id: &str) -> Option<String> {
  let normalized = id.trim();
  let ctx = FLOWS.read().unwrap();
  
  if ctx.contains_key(normalized) {
    ctx.get(normalized).cloned()
  }
  else {
    let path = format!("./data/{}/flow.json", normalized);
    
    let flow = read_file_json_to_string(path).expect("Não foi possível encontrar as ações globais");

    drop(ctx);
    set(normalized, &flow);
    Some(flow)
  }
}

fn set(id: &str, value: &str) {
  let mut ctx = FLOWS.write().unwrap();
  ctx.insert(id.trim().to_string(), value.trim().to_string());
}