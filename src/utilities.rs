use std::{io, thread};
use std::io::Write;
use std::time::Duration;
use crate::colors::{BOLD, C_RESET, CYAN};

pub fn delete_last_line() {
    print!("\x1b[1A");
    print!("\x1b[2K");
    io::stdout().flush().unwrap();
}

pub fn start_counter() {
    for i in (0..4).rev() {
        println!("              Starting in {BOLD}{CYAN}{i}s{C_RESET}");
        thread::sleep(Duration::from_secs(1));
        delete_last_line();
    }

    thread::sleep(Duration::from_millis(500));
    println!("           Starting Benchmark...");
}