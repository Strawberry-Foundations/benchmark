use std::{io, thread};
use std::io::Write;
use std::time::Duration;

pub fn delete_last_line() {
    print!("\x1b[1A");
    print!("\x1b[2K");
    io::stdout().flush().unwrap();
}

pub fn start_counter() {
    println!("            Starting in 3");
    thread::sleep(Duration::from_secs(1));
    delete_last_line();

    println!("            Starting in 2");
    thread::sleep(Duration::from_secs(1));
    delete_last_line();

    println!("            Starting in 1");
    thread::sleep(Duration::from_secs(1));
    delete_last_line();

    thread::sleep(Duration::from_secs(1));
}