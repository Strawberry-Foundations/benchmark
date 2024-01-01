#![warn(clippy::all, clippy::nursery, clippy::pedantic)]
#![allow(clippy::match_same_arms, dead_code)]

mod colors;
mod utilities;
mod engines;
mod cli_wins;
mod cli;
mod type_modes;

use crate::cli::args::Args;
use crate::cli::sections::help_section;
use crate::engines::benchmark::Benchmark;
use crate::type_modes::{CPUMode, Modes};

pub const VERSION: &str = "1.2.0";

fn main() {
    let mut args = Args::new();
    let collector = args.collector();

    let mut benchmark = Benchmark::new();

    for (index, arg) in collector.iter().enumerate() {
        match arg.as_str() {
            "--help"           => help_section(),
            "-n" | "--numbers" => benchmark.mode = Modes::NUMBERS,
            "-g" | "--graphic" => benchmark.mode = Modes::GRAPHIC,
            "-m" | "--multi"   => benchmark.num_counter.threading_mode = CPUMode::MULTI,
            "-s" | "--single"  => benchmark.num_counter.threading_mode = CPUMode::SINGLE,
            "-c" | "--counter" => benchmark.num_counter.counter = true,
            "-h" | "--threads" => benchmark.num_counter.threads = collector.get(index + 1)
                                                               .map_or(10,|val| val.parse::<u8>().map_or(10, |cpu_threads| cpu_threads)),
            _    => {  }
        };

        if arg.as_str() == "-t" {
            benchmark.benchmark_duration = collector.get(index + 1).map_or(10, |val| val.parse::<u64>().map_or(10, |parsed_time| parsed_time));
        }
    }

    if matches!(benchmark.mode, Modes::NUMBERS) {
        cli_wins::num::startup(benchmark.benchmark_duration, &benchmark.num_counter.threading_mode);
    }

    utilities::start_counter();

    match benchmark.mode {
        Modes::NUMBERS => args.bench_num(
            benchmark.benchmark_duration,
            &benchmark.num_counter.threading_mode,
            benchmark.num_counter.threads,
            benchmark.num_counter.counter
        ),

        Modes::GRAPHIC | Modes::NONE => { },
    }
}