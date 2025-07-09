use domain::contexts::Manager;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::RwLock;

pub static INPUT_CONTEXT: Lazy<RwLock<HashMap<String, String>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

pub struct InputManager {}

impl InputManager {
    pub fn new() -> Self {
        InputManager {}
    }
}

impl Manager for InputManager {
    fn get(&self, key: &str) -> Option<String> {
        let context = INPUT_CONTEXT.read().unwrap();
        context.get(key).cloned()
    }

    fn set(&self, key: &str, value: &str) {
        let mut context = INPUT_CONTEXT.write().unwrap();
        context.insert(key.trim().to_string(), value.trim().to_string());
    }

    fn reset(&self) {
        if let Ok(mut context) = INPUT_CONTEXT.write() {
            context.clear();
        } else {
            println!("Não foi possível resetar o contexto");
        }
    }
}
