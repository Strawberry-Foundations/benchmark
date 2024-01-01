use crate::cli_wins::time::return_bench_time;
use crate::colors::{BOLD, C_RESET, GREEN, RESET};
use crate::type_modes::Modes;
use crate::VERSION;

pub fn startup(bench_time: u64, modes: &Modes) {
    println!("\n{BOLD}  {GREEN}* ---- Benchmarking Tool v{VERSION} ---- *{RESET}{C_RESET}");
    return_bench_time(bench_time);
    println!("{BOLD}  {GREEN}|{RESET}    This tool will only use one     {GREEN}|{RESET}{C_RESET}");
    println!("{BOLD}  {GREEN}|{RESET}              CPU core!             {GREEN}|{RESET}{C_RESET}");
    println!("{BOLD}  {GREEN}* ---------------------------------- *{GREEN}{C_RESET}\n");

    println!("{:?}", modes);
}