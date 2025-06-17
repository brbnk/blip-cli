use types::flow::Flow;

use std::fs::File;
use std::io::Read;
use serde_json;

fn main() {
    let file_path = "./flow.json";
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Failed to read file");
    let flow: Flow = serde_json::from_str(&contents).expect("Failed to parse JSON");
    flow.get_start_state();
}