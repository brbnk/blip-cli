use crate::system;
use domain::constants;
use domain::contexts::Manager;
use domain::file_handler::Reader;
use file_handler::types::DataFile;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::RwLock;

pub static GLOBAL_ACTIONS: Lazy<RwLock<HashMap<String, String>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

pub struct GlobalActionsManager {}

impl GlobalActionsManager {
    pub fn new() -> Self {
        GlobalActionsManager {}
    }
}

impl Manager for GlobalActionsManager {
    fn get(&self, key: &str) -> Option<String> {
        let context = GLOBAL_ACTIONS.read().unwrap();
        let tenant = system::get_tenant();
        let key_normalized = key.trim();

        if context.contains_key(key_normalized) {
            context.get(key_normalized).cloned()
        } else {
            drop(context);

            let global_actions_file = DataFile {
                tenant: tenant,
                bot_id: Some(key_normalized.to_string()),
                file_name: String::from(constants::GLOBAL_ACTIONS_FILE_NAME),
                content: None,
            };

            let global_actions = global_actions_file
                .read()
                .expect(constants::GLOBAL_ACTIONS_FILE_NAME);

            self.set(key_normalized, &global_actions);

            Some(global_actions)
        }
    }

    fn set(&self, key: &str, value: &str) {
        let mut context: std::sync::RwLockWriteGuard<'_, HashMap<String, String>> =
            GLOBAL_ACTIONS.write().unwrap();
        context.insert(key.trim().to_string(), value.trim().to_string());
    }

    fn reset(&self) {
        todo!()
    }
}
