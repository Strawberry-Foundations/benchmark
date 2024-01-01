use crate::colors::{C_RESET, GREEN, BOLD, UNDERLINE, CYAN, RESET, WHITE, RED, MAGENTA};
use crate::VERSION;

pub fn help_section() {
    println!("\
{BOLD}{CYAN}{UNDERLINE}Benchmarking Tool v{VERSION}{C_RESET}\n\
{GREEN}{BOLD}Usage:{RESET} {WHITE}benchmark {CYAN}[command] {RED}[<options>]{C_RESET}\n\n\
{MAGENTA}{BOLD}Modes:{C_RESET}
    {CYAN}{BOLD}-n, --numbers:{C_RESET} Number Benchmark Engine {GREEN}{BOLD}[default]{C_RESET}
     {BOLD}↳ {MAGENTA}Options:{C_RESET}
            {CYAN}{BOLD}-s, --single{C_RESET}       Run Number Benchmark Single-threaded {GREEN}{BOLD}[default]{C_RESET}
            {CYAN}{BOLD}-m, --multi{C_RESET}        Run Number Benchmark Multi-threaded
            {CYAN}{BOLD}-h, --threads  <n>{C_RESET} Specify the number of threads used for multithreading
            {CYAN}{BOLD}-c, --counter{C_RESET}      Show number counter

");
    std::process::exit(0);
}