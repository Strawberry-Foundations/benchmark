use crate::colors::{C_RESET, GREEN, BOLD};

pub fn help_section() {
    println!("{BOLD}{GREEN}Benchmarking Tool v1.1.0{C_RESET}");
    std::process::exit(0);
}