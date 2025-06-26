use once_cell::sync::Lazy;
use std::collections::{HashMap};
use std::fs::File;
use std::io::Read;
use std::sync::RwLock;

pub static GLOBAL_ACTIONS: Lazy<RwLock<HashMap<String, String>>> = Lazy::new(|| {
  RwLock::new(HashMap::new())
});

pub fn get(id: &str) -> Option<String> {
  let ctx = GLOBAL_ACTIONS.read().unwrap();

  let normalized = id.trim();
  if ctx.contains_key(normalized) {
    ctx.get(normalized).cloned()
  }
  else {
    let path = format!("./data/{}/global_actions.json", normalized);

    let global_action = read_json_to_string(path).expect("Não foi possível encontrar as ações globais");
    
    drop(ctx);
    set(normalized, &global_action);
    Some(global_action)
  }
}

fn set(id: &str, value: &str) {
  let mut ctx = GLOBAL_ACTIONS.write().unwrap();
  ctx.insert(id.trim().to_string(), value.trim().to_string());
}

fn read_json_to_string(path: String) -> Result<String, String> {
    let mut file = File::open(&path)
        .map_err(|e| format!("Erro ao abrir o arquivo {}: {}", path, e))?;

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .map_err(|e| format!("Erro ao ler o arquivo: {}", e))?;

    Ok(contents)
}