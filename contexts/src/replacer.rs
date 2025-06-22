use crate::input::INPUT_CONTEXT;
use crate::user::USER_CONTEXT;

pub fn replace(message: &str) -> String {
  let mut result = message.to_string();

  let input = INPUT_CONTEXT.read().unwrap();
  let user = USER_CONTEXT.read().unwrap();
  
  let iterator = input
    .iter()
    .chain(user
      .iter());

  for (key, value) in iterator {
    let placeholder = format!("{{{{{}}}}}", key);
    result = result.replace(&placeholder, value);
  }

  result
}
