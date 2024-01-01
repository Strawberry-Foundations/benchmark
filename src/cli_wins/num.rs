use std::num::TryFromIntError;
use crate::cli_wins::time::return_bench_time;
use crate::colors::{BOLD, C_RESET, CYAN, GREEN, MAGENTA, RESET};
use crate::type_modes::CPUMode;
use crate::VERSION;

use std::thread::available_parallelism;
// use raw_cpuid::CpuId;

pub fn print_cpu_mode(cpu_mode: &CPUMode) {
    match cpu_mode {
        CPUMode::SINGLE => println!("{BOLD}  {GREEN}|{RESET}          {MAGENTA}Numbers (Single)          {GREEN}|{RESET}{C_RESET}"),
        CPUMode::MULTI  => println!("{BOLD}  {GREEN}|{RESET}          {MAGENTA}Numbers (Multi)           {GREEN}|{RESET}{C_RESET}"),
    }
}

pub fn get_logical_processors(cores: Result<u8, TryFromIntError>) {
    let cores = cores.unwrap().to_string();

    match cores.len() {
        1 => println!("{BOLD}  {GREEN}|{RESET}        Logical Processors: {CYAN}{cores}       {GREEN}|{RESET}{C_RESET}"),
        2 => println!("{BOLD}  {GREEN}|{RESET}       Logical Processors: {CYAN}{cores}       {GREEN}|{RESET}{C_RESET}"),
        3 => println!("{BOLD}  {GREEN}|{RESET}       Logical Processors: {CYAN}{cores}      {GREEN}|{RESET}{C_RESET}"),
        4 => println!("{BOLD}  {GREEN}|{RESET}      Logical Processors: {CYAN}{cores}      {GREEN}|{RESET}{C_RESET}"),
        _ => println!("{BOLD}  {GREEN}|{RESET}      Logical Processors: {CYAN}....      {GREEN}|{RESET}{C_RESET}"),
    }

}

pub fn startup(bench_time: u64, cpu_mode: &CPUMode) {
    println!("\n{BOLD}  {GREEN}* ---- Benchmarking Tool v{VERSION} ---- *{RESET}{C_RESET}");
    return_bench_time(bench_time);
    get_logical_processors(u8::try_from(available_parallelism().unwrap().get()));
    println!("{BOLD}  {GREEN}|{RESET}                                    {GREEN}|{RESET}{C_RESET}");
    println!("{BOLD}  {GREEN}|{RESET}           Current mode:            {GREEN}|{RESET}{C_RESET}");
    print_cpu_mode(cpu_mode);
    println!("{BOLD}  {GREEN}* ---------------------------------- *{GREEN}{C_RESET}\n");

    /*
    let cpuid = CpuId::new();
    let binding = cpuid.get_processor_brand_string().unwrap();
    let cpu_string = binding.as_str();
    
    println!("{BOLD}{MAGENTA}CPU: {C_RESET}{}", cpu_string);
    */
}