use crate::{configs, contact, context, input, resource};

pub fn get(key: &str) -> Option<String> {
  let input = input::get(key);
  if input.is_some() {
    return Some(input.unwrap());
  }

  let config = configs::get(key);
  if config.is_some() {
    return Some(config.unwrap());
  }

  let resource = resource::get(key);
  if resource.is_some() {
    return Some(resource.unwrap());
  }

  let user = contact::get(key);
  if user.is_some() {
    return Some(user.unwrap());
  }

  let user_context = context::get(key);
  if user_context.is_some() {
    return Some(user_context.unwrap());
  }

  return None;
}
