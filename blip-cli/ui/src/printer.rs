use crate::types::{ActionProps, Color};
use colored::{ColoredString, Colorize};

pub fn print_action(props: ActionProps) {
    let r_key = &props.key;
    let r_value = &props.value;
    println!(
        "{} {:<13}{} {} {} {}",
        colorize("|", Color::BrightBlack),
        colorize(&props.name, props.color),
        colorize("|", Color::BrightBlack),
        r_key,
        colorize("->", Color::BrightBlack),
        r_value
    );
}

pub fn println(message: &str, color: Color) {
    println!("{}", colorize(message, color));
}

pub fn print(message: &str, color: Color) {
    print!("{}", colorize(message, color));
}

pub fn print_state_title(title: &str) {
    let length = 60;
    let min_length = title.len() + 2;
    let length = length.max(min_length);

    let upper_state = format!("+{}+", "-".repeat(length - 2));
    let total_padding = length - 2 - title.len();
    let left_padding = total_padding / 2;
    let right_padding = total_padding - left_padding;

    println!("{}", colorize(&upper_state, Color::BrightBlack));
    println!(
        "{}{}{}{}{}",
        colorize("|", Color::BrightBlack),
        " ".repeat(left_padding),
        colorize(title, Color::Green),
        " ".repeat(right_padding),
        colorize("|", Color::BrightBlack)
    );
    println!("{}", colorize(&upper_state, Color::BrightBlack));
}

pub fn print_success_message(message: &str) {
    println!(
        "[{}] {}", 
        colorize("Ok", Color::Green),
        colorize(message, Color::Yellow)
    );
}

pub fn print_error_message(message: &str) {
    println!(
        "[{}] {}", 
        colorize("ERRO", Color::Red),
        colorize(message, Color::Yellow)
    );
}

pub fn print_test_message(message: &str, is_success: bool) {
    let emoji = if is_success { "✔️" } else { "❌" };
    println!("{} {}", emoji, message);
}

pub fn y(text: &str) -> ColoredString {
    text.yellow()
}

pub fn b(text: &str) -> ColoredString {
    text.blue()
}

pub fn colorize(text: &str, color: Color) -> ColoredString {
    match color {
        Color::Yellow => text.yellow(),
        Color::Red => text.red(),
        Color::Blue => text.blue(),
        Color::Cyan => text.cyan(),
        Color::Black => text.black(),
        Color::Magenta => text.magenta(),
        Color::White => text.white(),
        Color::BrightBlack => text.bright_black(),
        Color::Green => text.green(),
    }
}