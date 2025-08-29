use domain::contexts::Manager;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::RwLock;

pub static EVENT_CONTEXT: Lazy<RwLock<HashMap<String, String>>> = Lazy::new(|| {
    let context = HashMap::new();
    RwLock::new(context)
});

pub struct EventManager {}

impl Manager for EventManager {
    fn get(&self, key: &str) -> Option<String> {
        let pool = EVENT_CONTEXT.read().unwrap();

        let result = match pool.get(key) {
            Some(events) => Some(events.clone()),
            None => None,
        };

        drop(pool);
        result
    }

    fn set(&self, key: &str, value: &str) {
        let mut events: Vec<String> = match self.get(key) {
            Some(events_str) => {
                file_handler::deserialize::<Vec<String>>(&events_str).expect("events deserialized")
            }
            None => Vec::new(),
        };
        let mut context = EVENT_CONTEXT.write().unwrap();

        events.push(value.to_owned());

        context.insert(
            key.trim().to_string(),
            serde_json::to_string(&events).expect("serialized events"),
        );
    }

    fn delete(&self, key: &str) {
        let mut writer = EVENT_CONTEXT.write().unwrap();
        writer.remove_entry(key);
    }

    fn reset(&self) {
        if let Ok(mut context) = EVENT_CONTEXT.write() {
            context.clear();
        } else {
            println!("Não foi possível resetar o contexto");
        }
    }
}

impl EventManager {
    pub fn new() -> Self {
        EventManager {}
    }
}
