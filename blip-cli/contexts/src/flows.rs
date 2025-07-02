use crate::context;
use json_converter::read_file_json_to_string;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::RwLock;

pub static FLOWS: Lazy<RwLock<HashMap<String, String>>> = Lazy::new(|| RwLock::new(HashMap::new()));

pub fn get(id: &str) -> Option<String> {
    let tenant = context::get_tenant();
    let id_normalized = id.trim();
    let ctx = FLOWS.read().unwrap();

    if ctx.contains_key(id_normalized) {
        ctx.get(id_normalized).cloned()
    } else {
        let path = format!("./data/{}/{}/flow.json", &tenant, id_normalized);

        let flow =
            read_file_json_to_string(path).expect("Não foi possível encontrar as ações globais");

        drop(ctx);
        set(id_normalized, &flow);
        Some(flow)
    }
}

fn set(id: &str, value: &str) {
    let mut ctx = FLOWS.write().unwrap();
    ctx.insert(id.trim().to_string(), value.trim().to_string());
}
