use domain::traits::contexts::Manager;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::RwLock;

pub static CONTACT_CONTEXT: Lazy<RwLock<HashMap<String, String>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

pub struct ContactManager {}

impl ContactManager {
    pub fn new() -> Self {
        ContactManager {}
    }
}

impl Manager for ContactManager {
    fn get(&self, key: &str) -> Option<String> {
        let context = CONTACT_CONTEXT.read().unwrap();
        context.get(key).cloned()
    }

    fn set(&self, key: &str, value: &str) {
        let mut context = CONTACT_CONTEXT.write().unwrap();
    
        context.insert(
            key.trim().to_string(), 
            value.trim().to_string());
    }
    
    fn reset(&self) {
        if let Ok(mut context) = CONTACT_CONTEXT.write() {
            context.clear();
        }
        else {
            println!("Não foi possível resetar o contexto");
        }
    }
}