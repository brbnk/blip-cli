use crate::{input, contact, context};

pub fn get(key: &str) -> Option<String> {
  let input = input::get(key);

  if input.is_some() {
    return Some(input.unwrap());
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
