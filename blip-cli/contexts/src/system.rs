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
const IS_ROUTER_KEY: &str = "sys.is_router";
const ROUTER_ID_KEY: &str = "sys.router_id";
const ROUTER_CHILDREN: &str = "sys.router_children";
const REDIRECT_KEY: &str = "sys.redirect";
const REDIRECT_SIGNAL_KEY: &str = "sys.redirect_signal";
const REDIRECT_TRANSITION_SIGNAL_KEY: &str = "sys.redirect_transition_signal";

// Tenant
pub fn get_tenant() -> String {
    match MANAGER_POOL.context.get(TENANT_KEY) {
        Some(tenant) => tenant,
        None => panic!("tenant não encontrado"),
    }
}

pub fn set_tenant(tenant: &str) {
    MANAGER_POOL.context.set(TENANT_KEY, tenant);
}

// Master-State
pub fn get_master_state() -> String {
    match MANAGER_POOL.context.get(MASTER_STATE) {
        Some(ms) => ms,
        None => panic!("master-state não encontrado"),
    }
}

pub fn set_master_state(id: &str) {
    MANAGER_POOL.context.set(MASTER_STATE, id);
}

// State Previous Id
pub fn get_state_previous_id() -> String {
    match MANAGER_POOL.context.get(STATE_PREVIOUS_ID) {
        Some(s) => s,
        None => panic!("state.previous.id não encontrado"),
    }
}

pub fn set_state_previous_id(id: &str) {
    MANAGER_POOL.context.set(STATE_PREVIOUS_ID, id);
}

// State Previous Name
pub fn get_state_previous_name() -> String {
    match MANAGER_POOL.context.get(STATE_PREVIOUS_NAME) {
        Some(s) => s,
        None => panic!("state.previous.name não encontrado"),
    }
}

pub fn set_state_previous_name(id: &str) {
    MANAGER_POOL.context.set(STATE_PREVIOUS_NAME, id);
}

// State Id
pub fn get_state_id() -> String {
    match MANAGER_POOL.context.get(STATE_ID) {
        Some(s) => s,
        None => panic!("state.id não encontrado"),
    }
}

pub fn set_state_id(id: &str) {
    MANAGER_POOL.context.set(STATE_ID, id);
}

// State Name
pub fn get_state_name() -> String {
    match MANAGER_POOL.context.get(STATE_NAME) {
        Some(s) => s,
        None => panic!("state.name não encontrado"),
    }
}

pub fn set_state_name(id: &str) {
    MANAGER_POOL.context.set(STATE_NAME, id);
}

// Test
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

// Router
pub fn set_router_id(id: &str) {
    MANAGER_POOL.context.set(ROUTER_ID_KEY, id);
}

pub fn get_router_children() -> Option<String> {
    match MANAGER_POOL.context.get(ROUTER_CHILDREN) {
        Some(children) => {
            if !children.is_empty() {
                Some(children)
            } else {
                None
            }
        }
        None => None,
    }
}

pub fn set_router_children(children: &str) {
    MANAGER_POOL.context.set(ROUTER_CHILDREN, children);
}

pub fn get_router_id() -> Option<String> {
    match MANAGER_POOL.context.get(ROUTER_ID_KEY) {
        Some(id) => {
            if !id.is_empty() {
                Some(id)
            } else {
                None
            }
        }
        None => None,
    }
}

pub fn set_router_mode() {
    MANAGER_POOL.context.set(IS_ROUTER_KEY, "true");
}

pub fn is_router() -> bool {
    match MANAGER_POOL.context.get(IS_ROUTER_KEY) {
        Some(value) => value.eq("true"),
        None => false,
    }
}

// Redirect
pub fn has_redirect() -> bool {
    match get_redirect() {
        Some(_) => true,
        None => false,
    }
}

pub fn clear_redirect() {
    set_redirect("");
    reset_redirect_signal();
    reset_redirect_transition_signal();
}

pub fn set_redirect(redirect: &str) {
    MANAGER_POOL.context.set(REDIRECT_KEY, redirect);
}

pub fn get_redirect() -> Option<String> {
    match MANAGER_POOL.context.get(REDIRECT_KEY) {
        Some(redirect) => {
            if !redirect.is_empty() {
                Some(redirect)
            } else {
                None
            }
        }
        None => None,
    }
}

pub fn set_redirect_signal() {
    MANAGER_POOL.context.set(REDIRECT_SIGNAL_KEY, "true")
}

pub fn reset_redirect_signal() {
    MANAGER_POOL.context.set(REDIRECT_SIGNAL_KEY, "false")
}

pub fn is_redirect_signal() -> bool {
    match MANAGER_POOL.context.get(REDIRECT_SIGNAL_KEY) {
        Some(mode) => mode.eq_ignore_ascii_case("true"),
        None => false,
    }
}

pub fn set_redirect_transition_signal(context: &str) {
    let value = match !context.eq("no_context") {
        true => "true",
        false => "false",
    };

    MANAGER_POOL.context.set(REDIRECT_TRANSITION_SIGNAL_KEY, value)
}

pub fn reset_redirect_transition_signal() {
    MANAGER_POOL.context.set(REDIRECT_TRANSITION_SIGNAL_KEY, "false")
}

pub fn is_redirect_transition_signal() -> bool {
    match MANAGER_POOL.context.get(REDIRECT_TRANSITION_SIGNAL_KEY) {
        Some(mode) => mode.eq_ignore_ascii_case("true"),
        None => false,
    }
}
