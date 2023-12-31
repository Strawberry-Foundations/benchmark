use crate::colors::{BOLD, C_RESET, CYAN, GREEN, RESET};
use crate::VERSION;

pub fn return_bench_time(bench_time: &u64) {
    match bench_time.to_string().len() {
        1 => println!("{BOLD}  {GREEN}|{RESET}       Set benchmark time: {CYAN}{bench_time}s       {GREEN}|{RESET}{C_RESET}"),
        2 => println!("{BOLD}  {GREEN}|{RESET}       Set benchmark time: {CYAN}{bench_time}s      {GREEN}|{RESET}{C_RESET}"),
        3 => println!("{BOLD}  {GREEN}|{RESET}      Set benchmark time: {CYAN}{bench_time}s      {GREEN}|{RESET}{C_RESET}"),
        4 => println!("{BOLD}  {GREEN}|{RESET}      Set benchmark time: {CYAN}{bench_time}s     {GREEN}|{RESET}{C_RESET}"),
        5 => println!("{BOLD}  {GREEN}|{RESET}     Set benchmark time: {CYAN}{bench_time}s     {GREEN}|{RESET}{C_RESET}"),
        _ => std::process::exit(1),
    }
}

pub fn startup(bench_time: &u64) {
    println!("\n{BOLD}  {GREEN}* ---- Benchmarking Tool v{VERSION} ---- *{RESET}{C_RESET}");
    return_bench_time(bench_time);
    println!("{BOLD}  {GREEN}|{RESET}    This tool will only use one     {GREEN}|{RESET}{C_RESET}");
    println!("{BOLD}  {GREEN}|{RESET}            cpu core!               {GREEN}|{RESET}{C_RESET}");
    println!("{BOLD}  {GREEN}* ---------------------------------- *{GREEN}{C_RESET}\n");
}

