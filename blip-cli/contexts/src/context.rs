use domain::contexts::Manager;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::RwLock;

use crate::MANAGER_POOL;

pub static CONTEXT: Lazy<RwLock<HashMap<String, String>>> = Lazy::new(|| {
    let context = HashMap::new();
    RwLock::new(context)
});

pub struct ContextManager {}

impl Manager for ContextManager {
    fn get(&self, key: &str) -> Option<String> {
        let context = CONTEXT.read().unwrap();

        match MANAGER_POOL.mocks.get(key) {
            Some(mock) => {
                drop(context);
                self.set(key, &mock);
                Some(mock)
            },
            None => context.get(key).cloned(),
        }
    }

    fn set(&self, key: &str, value: &str) {
        let mut context = CONTEXT.write().unwrap();

        match MANAGER_POOL.mocks.get(key) {
            Some(v) => {
                context.insert(key.trim().to_string(), v.trim().to_string());
            }
            None => {
                context.insert(key.trim().to_string(), value.trim().to_string());
            },
        }
    }

    fn delete(&self, key: &str) {
        let mut writer = CONTEXT.write().unwrap();
        writer.remove_entry(key);
    }

    fn reset(&self) {
        if let Ok(mut context) = CONTEXT.write() {
            context.clear();
        } else {
            println!("Não foi possível resetar o contexto");
        }
    }
}

impl ContextManager {
    pub fn new() -> Self {
        ContextManager {}
    }
}
