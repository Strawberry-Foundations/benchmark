use crate::colors::{BOLD, C_RESET, CYAN, GREEN, RESET};

pub fn return_result(bench_time: u64) {
    match bench_time.to_string().len() {
        1 => println!("{BOLD}  {GREEN}|{RESET}                  {CYAN}{bench_time}                 {GREEN}|{RESET}{C_RESET}"),
        2 => println!("{BOLD}  {GREEN}|{RESET}                 {CYAN}{bench_time}                 {GREEN}|{RESET}{C_RESET}"),
        3 => println!("{BOLD}  {GREEN}|{RESET}                {CYAN}{bench_time}                 {GREEN}|{RESET}{C_RESET}"),
        4 => println!("{BOLD}  {GREEN}|{RESET}                {CYAN}{bench_time}                {GREEN}|{RESET}{C_RESET}"),
        5 => println!("{BOLD}  {GREEN}|{RESET}                {CYAN}{bench_time}               {GREEN}|{RESET}{C_RESET}"),
        6 => println!("{BOLD}  {GREEN}|{RESET}               {CYAN}{bench_time}               {GREEN}|{RESET}{C_RESET}"),
        _ => println!("{BOLD}  {GREEN}|{RESET}            over {CYAN}999.999            {GREEN}|{RESET}{C_RESET}"),
    }
}

pub fn result(result: u64) {
    println!("\n{BOLD}  {GREEN}* ----- Your results are here! ----- *{RESET}{C_RESET}");
    println!("{BOLD}  {GREEN}|{RESET}      Your computer has scored      {GREEN}|{RESET}{C_RESET}");
    return_result(result);
    println!("{BOLD}  {GREEN}|{RESET}               points!              {GREEN}|{RESET}{C_RESET}");
    println!("{BOLD}  {GREEN}* ---------------------------------- *{GREEN}{C_RESET}\n");
}