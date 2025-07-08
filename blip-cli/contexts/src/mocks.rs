use domain::traits::contexts::Manager;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::RwLock;

pub static MOCKS: Lazy<RwLock<HashMap<String, String>>> = Lazy::new(|| {
    let context = HashMap::new();
    RwLock::new(context)
});

pub struct MocksManager {}

impl Manager for MocksManager {
    fn get(&self, key: &str) -> Option<String> {
        let context = MOCKS.read().unwrap();

        let value = context
            .get(key)
            .cloned();

        drop(context);
        let mut writer = MOCKS.write().unwrap();
        writer.remove_entry(key);

        value
    }

    fn set(&self, key: &str, value: &str) {
        let mut context = MOCKS.write().unwrap();
    
        context.insert(
            key.trim().to_string(), 
            value.trim().to_string());
    }

    fn reset(&self) {
        if let Ok(mut context) = MOCKS.write() {
            context.clear();
        }
        else {
            println!("Não foi possível resetar o contexto");
        }
    }
}

impl MocksManager {
    pub fn new() -> Self {
        MocksManager {}
    }
}
