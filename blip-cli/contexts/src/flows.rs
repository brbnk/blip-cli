use crate::system;
use domain::constants;
use domain::contexts::Manager;
use domain::file_handler::Reader;
use file_handler::types::DataFile;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::RwLock;

pub static FLOWS: Lazy<RwLock<HashMap<String, String>>> = Lazy::new(|| RwLock::new(HashMap::new()));

pub struct FlowsManager {}

impl FlowsManager {
    pub fn new() -> Self {
        FlowsManager {}
    }
}

impl Manager for FlowsManager {
    fn get(&self, key: &str) -> Option<String> {
        let tenant = system::get_tenant();
        let key_normalized = key.trim();
        let context: std::sync::RwLockReadGuard<'_, HashMap<String, String>> =
            FLOWS.read().unwrap();

        if context.contains_key(key_normalized) {
            context.get(key_normalized).cloned()
        } else {
            drop(context);

            let flow_file = DataFile {
                tenant: tenant,
                bot_id: Some(key_normalized.to_string()),
                file_name: String::from(constants::FLOW_FILE_NAME),
                content: None,
            };

            let flow = flow_file.read().expect(constants::FLOW_FILE_NAME);

            self.set(key_normalized, &flow);

            Some(flow)
        }
    }

    fn set(&self, key: &str, value: &str) {
        let mut context = FLOWS.write().unwrap();

        context.insert(key.trim().to_string(), value.trim().to_string());
    }

    fn reset(&self) {
        todo!()
    }
}
