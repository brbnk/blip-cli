use crate::{configs::ConfigManager, MANAGER_POOL};
use domain::contexts::Manager;

pub fn get(key: &str) -> Option<String> {
    let input = MANAGER_POOL.input.get(key);
    if input.is_some() {
        return Some(input.unwrap());
    }

    let config_value = ConfigManager::new().get(key);
    if config_value.is_some() {
        return config_value;
    }

    let resource = MANAGER_POOL.resource.get(key);
    if resource.is_some() {
        return Some(resource.unwrap());
    }

    let contact_value = MANAGER_POOL.contact.get(key);
    if contact_value.is_some() {
        return contact_value;
    }

    let context_value = MANAGER_POOL.context.get(key);
    if context_value.is_some() {
        return context_value;
    }

    return None;
}

pub fn reset() {
    MANAGER_POOL.input.reset();
    MANAGER_POOL.context.reset();
    MANAGER_POOL.contact.reset();
    MANAGER_POOL.event.reset();
}
