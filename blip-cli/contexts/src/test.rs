use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::RwLock;

const TEST_MODE_KEY: &str = "sys.test_mode";
const TEST_INPUTS_KEY: &str = "sys.test_inputs";
const END_OF_SIGNAL_KEY: &str = "sys.end_of_test_signal";

pub static TEST_CONTEXT: Lazy<RwLock<HashMap<String, String>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

pub fn get(key: &str) -> Option<String> {
    let ctx = TEST_CONTEXT.read().unwrap();
    ctx.get(key).cloned()
}

pub fn set(key: &str, value: &str) {
    let mut ctx = TEST_CONTEXT.write().unwrap();
    ctx.insert(key.trim().to_string(), value.trim().to_string());
}

pub fn reset() {
    if let Ok(mut context) = TEST_CONTEXT.write() {
        context.clear();
    }
    else {
        println!("Não foi possível resetar o contexto");
    }
}

pub fn enter_mode() {
    set(TEST_MODE_KEY, "true")
}

pub fn is_activated() -> bool {
    match get(TEST_MODE_KEY) {
        Some(mode) => mode.eq_ignore_ascii_case("true"),
        None => false,
    }
}

pub fn set_end_signal() {
    set(END_OF_SIGNAL_KEY, "true")
}

pub fn reset_end_signal() {
    set(END_OF_SIGNAL_KEY, "false")
}

pub fn is_reset_end_signal() -> bool {
    match get(END_OF_SIGNAL_KEY) {
        Some(mode) => mode.eq_ignore_ascii_case("true"),
        None => false,
    }
}

pub fn set_inputs(inputs: &str) {
  set(TEST_INPUTS_KEY, &inputs);
}

pub fn get_inputs() -> Option<String> {
  match get(TEST_INPUTS_KEY) {
    Some(inputs) => {
      if !inputs.is_empty() {
        Some(inputs)
      }
      else {
        None
      }
    },
    None => None,
}
}