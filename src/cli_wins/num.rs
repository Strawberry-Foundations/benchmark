use crate::cli_wins::time::return_bench_time;
use crate::colors::{BOLD, C_RESET, GREEN, MAGENTA, RESET};
use crate::type_modes::{CPUMode};
use crate::VERSION;

pub fn print_cpu_mode(cpu_mode: &CPUMode) {
    match cpu_mode {
        CPUMode::SINGLE => println!("{BOLD}  {GREEN}|{RESET}          {MAGENTA}Numbers (Single)          {GREEN}|{RESET}{C_RESET}"),
        CPUMode::MULTI  => println!("{BOLD}  {GREEN}|{RESET}          {MAGENTA}Numbers (Multi)           {GREEN}|{RESET}{C_RESET}"),
    }
}

pub fn startup(bench_time: u64, cpu_mode: &CPUMode) {
    println!("\n{BOLD}  {GREEN}* ---- Benchmarking Tool v{VERSION} ---- *{RESET}{C_RESET}");
    return_bench_time(bench_time);
    println!("{BOLD}  {GREEN}|{RESET}                                    {GREEN}|{RESET}{C_RESET}");
    println!("{BOLD}  {GREEN}|{RESET}           Current mode:            {GREEN}|{RESET}{C_RESET}");
    print_cpu_mode(cpu_mode);
    println!("{BOLD}  {GREEN}* ---------------------------------- *{GREEN}{C_RESET}\n");
}