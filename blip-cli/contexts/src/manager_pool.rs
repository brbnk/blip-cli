use domain::contexts::Manager;
use once_cell::sync::Lazy;

use crate::{
    BlipFunctionsManager, ConfigManager, ContactManager, ContextManager, EventManager, FlowsManager, GlobalActionsManager, InputManager, MocksManager, ResourceManager, TestManager
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

    pub test: Box<dyn Manager>,

    pub event: Box<dyn Manager>,

    pub mocks: Box<dyn Manager>,

    pub blip_functions: Box<dyn Manager>
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
            test: Box::new(TestManager::new()),
            event: Box::new(EventManager::new()),
            mocks: Box::new(MocksManager::new()),
            blip_functions: Box::new(BlipFunctionsManager::new())
        }
    }
}
