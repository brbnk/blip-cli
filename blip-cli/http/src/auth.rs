use std::env;

use domain::constants::{TOKEN_KEY};
use ui::{printer, types::Color};

pub fn get_token() -> Option<String> {
    if let Ok(token) = env::var(TOKEN_KEY) {
        Some(token)
    } else {
        let error_msg = &format!("Configure a variável de ambiente ${} com o token obtido no Portal.", TOKEN_KEY);
        printer::println(&error_msg, Color::Yellow);
        None
    }
}
