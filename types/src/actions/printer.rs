use colored::{ColoredString, Colorize};
use contexts::{replacer};

fn print_action(action: ColoredString, key: &String, value: &String) {
    println!(
      "+ {}: {} -> {}", 
      action,
      replacer::replace(key), 
      replacer::replace(value));
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