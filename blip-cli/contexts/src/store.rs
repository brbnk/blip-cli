use crate::{configs::ConfigManager, MANAGER_POOL};
use domain::{constants::{CONFIG_VAR_PREFIX, RESOURCE_PREFIX}, contexts::Manager};

pub fn get(key: &str) -> Option<String> {
    let input = MANAGER_POOL.input.get(key);
    if input.is_some() {
        return Some(input.unwrap());
    }

    if key.contains(CONFIG_VAR_PREFIX) {
        let config_value = ConfigManager::new().get(key);
        if config_value.is_some() {
            return config_value;
        }
    }

    if key.contains(RESOURCE_PREFIX) {
        let resource = MANAGER_POOL.resource.get(key);
        if resource.is_some() {
            return Some(resource.unwrap());
        }
    }

    if key.contains(CONFIG_VAR_PREFIX) {
        let contact_value = MANAGER_POOL.contact.get(key);
        if contact_value.is_some() {
            return contact_value;
        }
    }

    let context_value = MANAGER_POOL.context.get(key);
    if context_value.is_some() {
        return context_value;
    }

    let blip_function_value = MANAGER_POOL.blip_functions.get(key);
    if blip_function_value.is_some() {
        return blip_function_value;
    }

    return None;
}

pub fn reset() {
    MANAGER_POOL.input.reset();
    MANAGER_POOL.context.reset();
    MANAGER_POOL.contact.reset();
    MANAGER_POOL.event.reset();
}
