use domain::traits::contexts::Manager;
use once_cell::sync::Lazy;

use crate::{
    ConfigManager, ContactManager, ContextManager, FlowsManager, GlobalActionsManager,
    InputManager, ResourceManager,
};

pub static MANAGER_POOL: Lazy<ManagerPool> = Lazy::new(|| ManagerPool::new());

pub struct ManagerPool {
    pub contact: Box<dyn Manager>,

    pub config: Box<dyn Manager>,

    pub context: Box<dyn Manager>,

    pub input: Box<dyn Manager>,

    pub resource: Box<dyn Manager>,

    pub flow: Box<dyn Manager>,

    pub global_actions: Box<dyn Manager>,
}

impl ManagerPool {
    pub fn new() -> ManagerPool {
        ManagerPool {
            contact: Box::new(ContactManager::new()),
            config: Box::new(ConfigManager::new()),
            context: Box::new(ContextManager::new()),
            input: Box::new(InputManager::new()),
            resource: Box::new(ResourceManager::new()),
            flow: Box::new(FlowsManager::new()),
            global_actions: Box::new(GlobalActionsManager::new()),
        }
    }
}
