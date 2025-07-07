use crate::MANAGER_POOL;

const TENANT_KEY: &str = "tenant";
const MASTER_STATE: &str = "master-state";
const STATE_PREVIOUS_ID: &str = "state.previous.id";
const STATE_PREVIOUS_NAME: &str = "state.previous.name";
const STATE_ID: &str = "state.id";
const STATE_NAME: &str = "state.name";
const TEST_MODE_KEY: &str = "sys.test_mode";
const TEST_INPUTS_KEY: &str = "sys.test_inputs";
const END_OF_SIGNAL_KEY: &str = "sys.end_of_test_signal";

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


pub fn enter_test_mode() {
    MANAGER_POOL.test.set(TEST_MODE_KEY, "true")
}

pub fn is_test_mode() -> bool {
    match MANAGER_POOL.test.get(TEST_MODE_KEY) {
        Some(mode) => mode.eq_ignore_ascii_case("true"),
        None => false,
    }
}

pub fn set_end_test_signal() {
    MANAGER_POOL.test.set(END_OF_SIGNAL_KEY, "true")
}

pub fn reset_end_test_signal() {
    MANAGER_POOL.test.set(END_OF_SIGNAL_KEY, "false")
}

pub fn is_reset_end_test_signal() -> bool {
    match MANAGER_POOL.test.get(END_OF_SIGNAL_KEY) {
        Some(mode) => mode.eq_ignore_ascii_case("true"),
        None => false,
    }
}

pub fn set_test_inputs(inputs: &str) {
    MANAGER_POOL.test.set(TEST_INPUTS_KEY, &inputs);
}

pub fn get_test_inputs() -> Option<String> {
    match MANAGER_POOL.test.get(TEST_INPUTS_KEY) {
        Some(inputs) => {
            if !inputs.is_empty() {
                Some(inputs)
            } else {
                None
            }
        }
        None => None,
    }
}