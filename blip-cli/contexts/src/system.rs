use crate::MANAGER_POOL;

const TENANT_KEY: &str = "tenant";
const MASTER_STATE: &str = "master-state";
const STATE_PREVIOUS_ID: &str = "state.previous.id";
const STATE_PREVIOUS_NAME: &str = "state.previous.name";
const STATE_ID: &str = "state.id";
const STATE_NAME: &str = "state.name";

pub fn get_tenant() -> String {
  match MANAGER_POOL.context.get(TENANT_KEY) {
    Some(tenant) => tenant,
    None => panic!("tenant não encontrado"),
  }
}

pub fn set_tenant(tenant: &str) {
  MANAGER_POOL.context.set(TENANT_KEY, tenant);
}

pub fn get_master_state() -> String {
  match MANAGER_POOL.context.get(MASTER_STATE) {
    Some(ms) => ms,
    None => panic!("master-state não encontrado"),
  }
}

pub fn set_master_state(id: &str) {
  MANAGER_POOL.context.set(MASTER_STATE, id);
}

pub fn get_state_previous_id() -> String {
  match MANAGER_POOL.context.get(STATE_PREVIOUS_ID) {
    Some(s) => s,
    None => panic!("state.previous.id não encontrado"),
  }
}

pub fn set_state_previous_id(id: &str) {
  MANAGER_POOL.context.set(STATE_PREVIOUS_ID, id);
}

pub fn get_state_previous_name() -> String {
  match MANAGER_POOL.context.get(STATE_PREVIOUS_NAME) {
    Some(s) => s,
    None => panic!("state.previous.name não encontrado"),
  }
}

pub fn set_state_previous_name(id: &str) {
  MANAGER_POOL.context.set(STATE_PREVIOUS_NAME, id);
}

pub fn get_state_id() -> String {
  match MANAGER_POOL.context.get(STATE_ID) {
    Some(s) => s,
    None => panic!("state.id não encontrado"),
  }
}

pub fn set_state_id(id: &str) {
  MANAGER_POOL.context.set(STATE_ID, id);
}

pub fn get_state_name() -> String {
  match MANAGER_POOL.context.get(STATE_NAME) {
    Some(s) => s,
    None => panic!("state.name não encontrado"),
  }
}

pub fn set_state_name(id: &str) {
  MANAGER_POOL.context.set(STATE_NAME, id);
}