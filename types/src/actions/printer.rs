use colored::{ColoredString, Colorize};
use contexts::{context, replacer};

fn print_action(action: ColoredString, key: &String, value: &String) {
    let should_execute_global_actions = context::get("sys.should_execute_global_actions");

    let break_line = match should_execute_global_actions {
        Some(is_global) => {
          if is_global.eq_ignore_ascii_case("false") {
            "\n"
          }
          else {
            "(Global Action)"
          }
        },
        None => "\n",
    };

    print!(
      "+ {}: {} -> {} {}", 
      action,
      replacer::replace(key), 
      replacer::replace(value),
      break_line);
}

pub fn print_yellow(action: &str, key: &String, value: &String) {
  print_action(action.yellow().bold(), key, value);
}

pub fn print_red(action: &str, key: &String, value: &String) {
  print_action(action.red().bold(), key, value);
}

pub fn print_blue(action: &str, key: &String, value: &String) {
  print_action(action.blue().bold(), key, value);
}

pub fn print_cyan(action: &str, key: &String, value: &String) {
  print_action(action.cyan().bold(), key, value);
}