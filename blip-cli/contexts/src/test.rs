use domain::contexts::Manager;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::RwLock;

pub static TEST_CONTEXT: Lazy<RwLock<HashMap<String, String>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

pub struct TestManager {}

impl TestManager {
    pub fn new() -> Self {
        TestManager {  }
    }
}

impl Manager for TestManager {
    fn get(&self, key: &str) -> Option<String> {
        let ctx = TEST_CONTEXT.read().unwrap();
        ctx.get(key).cloned()
    }

    fn set(&self, key: &str, value: &str) {
        let mut ctx = TEST_CONTEXT.write().unwrap();
        ctx.insert(key.trim().to_string(), value.trim().to_string());
    }

    fn delete(&self, key: &str) {
        let mut writer = TEST_CONTEXT.write().unwrap();
        writer.remove_entry(key);
    }

    fn reset(&self) {
        if let Ok(mut context) = TEST_CONTEXT.write() {
            context.clear();
        } else {
            println!("Não foi possível resetar o contexto");
        }
    }
}