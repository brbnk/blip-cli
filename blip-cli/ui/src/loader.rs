use std::{thread, time::Duration};

pub fn start(duration_in_sec: u32) {
    let frames = ["|", "/", "-", "\\"];
    let repetition: usize = (duration_in_sec * 10).try_into().unwrap();
    for i in 0..repetition {
        let frame = frames[i % frames.len()];
        print!("\r{}", frame);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
    print!("\x08 \x08");
}