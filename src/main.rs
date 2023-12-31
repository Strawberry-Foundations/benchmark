#![allow(dead_code)]

mod colors;
mod cli_wins;
mod utilities;
mod engines;

use std::env;

pub const VERSION: &str = "1.1.0";

fn main() {
    let args: Vec<String> = env::args().collect();
    let bench_time: u64 = if args.len() < 2 {
        10
    }
    else {
        args[1].parse().unwrap()
    };

    cli_wins::startup(&bench_time);

    utilities::start_counter();

    engines::num_counter::benchmark(bench_time);
}