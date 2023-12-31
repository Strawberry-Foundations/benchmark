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
    println!("              Starting in {BOLD}{CYAN}3s{C_RESET}");
    thread::sleep(Duration::from_secs(1));
    delete_last_line();

    println!("              Starting in {BOLD}{CYAN}2s{C_RESET}");
    thread::sleep(Duration::from_secs(1));
    delete_last_line();

    println!("              Starting in {BOLD}{CYAN}1s{C_RESET}");
    thread::sleep(Duration::from_secs(1));
    delete_last_line();

    println!("              Starting in {BOLD}{CYAN}0s{C_RESET}");
    thread::sleep(Duration::from_secs(1));
    delete_last_line();

    println!("           Starting Benchmark...");
}