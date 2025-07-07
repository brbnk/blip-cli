use domain::traits::contexts::Manager;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::RwLock;

pub static CONTEXT: Lazy<RwLock<HashMap<String, String>>> = Lazy::new(|| {
    let context = HashMap::new();
    RwLock::new(context)
});

pub struct ContextManager {}

impl Manager for ContextManager {
    fn get(&self, key: &str) -> Option<String> {
        let context = CONTEXT.read().unwrap();
    
        context
            .get(key)
            .cloned()
    }

    fn set(&self, key: &str, value: &str) {
        let mut context = CONTEXT.write().unwrap();
    
        context.insert(
            key.trim().to_string(), 
            value.trim().to_string());
    }

    fn reset(&self) {
        if let Ok(mut context) = CONTEXT.write() {
            context.clear();
        }
        else {
            println!("Não foi possível resetar o contexto");
        }
    }
}

impl ContextManager {
    pub fn new() -> Self {
        ContextManager {}
    }
}
