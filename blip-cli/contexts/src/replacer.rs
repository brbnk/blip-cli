use crate::store;
use regex::Regex;
use serde_json::{Value, from_str};

pub fn replace(message: &str) -> String {
    let mut result = message.to_string();

    result = replace_object(&result);

    result = replace_native_and_simple_vars(&result);

    result
}

fn replace_native_and_simple_vars(message: &str) -> String {
    let mut result = message.to_string();
    let re = Regex::new(r"\{\{(.+?)\}\}").unwrap();

    for cap in re.captures_iter(message) {
        let token: &str = &cap[1];
        let placeholder = format!("{{{{{}}}}}", token);

        if let Some(value) = store::get(token) {
            result = result.replace(&placeholder, &value);
        } else {
            result = result.replace(&placeholder, "");
        }
    }

    result
}

fn replace_object(message: &str) -> String {
    let mut result = message.to_string();
    let re = Regex::new(r"\{\{(.+?)\}\}").unwrap();

    for cap in re.captures_iter(message) {
        let token = &cap[1];
        if let Some((key, path)) = token.split_once('@') {
            let placeholder = format!("{{{{{}@{}}}}}", key, path);
            if let Some(obj_value) = store::get(key) {
                let json: Value = from_str(&obj_value).unwrap();
                if let Some(val) = get_json_path(&json, path) {
                    result =
                        result.replace(&placeholder, &val.to_string().trim_matches('"').to_string())
                }
            }
        }
    }

    result
}

fn get_json_path<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    let mut current = value;

    for key in path.split('.') {
        current = current.get(key)?;
    }

    Some(current)
}
