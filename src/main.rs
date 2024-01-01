#![allow(dead_code)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

mod colors;
mod utilities;
mod engines;
mod cli_wins;
mod cli;

use crate::cli::args::Args;

pub const VERSION: &str = "1.1.0";

fn main() {
    let mut args = Args::new();
    let _collector = args.collector();

    let bench_time = args.bench_time;

    cli_wins::main::startup(bench_time);

    utilities::start_counter();

    engines::num_counter::single::benchmark(bench_time, false); // might change this later
    // engines::num_counter::multi::benchmark(16, bench_time);
}