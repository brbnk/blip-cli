use crate::input::INPUT_CONTEXT;
use crate::contact::CONTACT_CONTEXT;
use crate::context::CONTEXT;

pub fn replace(message: &str) -> String {
  let mut result = message.to_string();

  let input = INPUT_CONTEXT.read().unwrap();
  let user = CONTACT_CONTEXT.read().unwrap();
  let context = CONTEXT.read().unwrap();

  let iterator = input
    .iter()
    .chain(user.iter())
    .chain(context.iter());

  for (key, value) in iterator {
    let placeholder = format!("{{{{{}}}}}", key);
    result = result.replace(&placeholder, value);
  }

  result
}
