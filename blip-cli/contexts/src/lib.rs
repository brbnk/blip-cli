pub mod replacer;
pub mod store;
pub mod system;

mod test;
mod input;
mod contact;
mod context;
mod global_actions;
mod flows;
mod configs;
mod resource;
mod manager_pool;
mod event;

pub use input::InputManager;
pub use contact::ContactManager;
pub use context::ContextManager;
pub use global_actions::GlobalActionsManager;
pub use flows::FlowsManager;
pub use configs::ConfigManager;
pub use resource::ResourceManager;
pub use test::TestManager;
pub use event::EventManager;
pub use manager_pool::MANAGER_POOL;