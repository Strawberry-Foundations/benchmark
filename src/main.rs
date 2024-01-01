#![allow(dead_code)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

extern crate core;

mod colors;
mod utilities;
mod engines;
mod cli_wins;

use std::env;

pub const VERSION: &str = "1.1.0";

fn main() {
    let args: Vec<String> = env::args().collect();
    let bench_time = args.get(1).map_or(10, |n| n.parse().unwrap());

    cli_wins::main::startup(bench_time);

    utilities::start_counter();

    // engines::num_counter::benchmark(bench_time, false); // might change this later
    engines::num_counter_mthread::benchmark(16, bench_time);
}