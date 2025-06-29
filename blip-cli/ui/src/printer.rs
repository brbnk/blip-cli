use colored::{ColoredString, Colorize};
use contexts::{replacer};

fn print_action(action: ColoredString, key: &String, value: &String) {
    println!(
      "{} {:<13}{} {} {} {}",
      "|".bright_black(),
      action,
      "|".bright_black(),
      replacer::replace(key), 
      "->".bright_black(),
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

pub fn print_black(action: &str, key: &String, value: &String) {
  print_action(action.black().bold(), key, value);
}

pub fn print_magenta(action: &str, key: &String, value: &String) {
  print_action(action.magenta().bold(), key, value);
}

pub fn print_state_title(title: &str) {
    let length = 60;
    let min_length = title.len() + 2;
    let length = length.max(min_length);

    let upper_state = format!("+{}+", "-".repeat(length - 2));
    let total_padding = length - 2 - title.len();
    let left_padding = total_padding / 2;
    let right_padding = total_padding - left_padding;

    println!("{}", upper_state.bright_black());
    println!(
        "{}{}{}{}{}",
        "|".bright_black(),
        " ".repeat(left_padding),
        title.bright_green().bold(),
        " ".repeat(right_padding),
        "|".bright_black()
    );
    println!("{}", upper_state.bright_black());
}