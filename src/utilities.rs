use std::io;
use std::io::Write;

pub fn delete_last_line() {
    print!("\x1b[1A");
    print!("\x1b[2K");
    io::stdout().flush().unwrap();
}

pub fn start_counter() {
    println!("            Starting in 3");
    delete_last_line();
    println!("            Starting in 2");
    delete_last_line();
    println!("            Starting in 1");
    delete_last_line();
}