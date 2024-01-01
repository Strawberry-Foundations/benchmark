use crate::colors::{BOLD, C_RESET, CYAN, GREEN, RED, RESET};

pub fn return_bench_time(bench_time: u64) {
    match bench_time.to_string().len() {
        1 => println!("{BOLD}  {GREEN}|{RESET}       Set benchmark time: {CYAN}{bench_time}s       {GREEN}|{RESET}{C_RESET}"),
        2 => println!("{BOLD}  {GREEN}|{RESET}       Set benchmark time: {CYAN}{bench_time}s      {GREEN}|{RESET}{C_RESET}"),
        3 => println!("{BOLD}  {GREEN}|{RESET}      Set benchmark time: {CYAN}{bench_time}s      {GREEN}|{RESET}{C_RESET}"),
        4 => println!("{BOLD}  {GREEN}|{RESET}      Set benchmark time: {CYAN}{bench_time}s     {GREEN}|{RESET}{C_RESET}"),
        5 => println!("{BOLD}  {GREEN}|{RESET}     Set benchmark time: {CYAN}{bench_time}s     {GREEN}|{RESET}{C_RESET}"),
        _ => {
            println!("{BOLD}{RED}ERROR: Something went wrong!");
            std::process::exit(1);
        },
    }
}