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
use crate::type_modes::{CPUMode, Modes};

pub const VERSION: &str = "1.1.0";

fn main() {
    let mut args = Args::new();
    let collector = args.collector();

    let mut mode: Modes             = Modes::NUMBERS;
    let mut bench_time: u64         = 10;

    let mut threading_mode: CPUMode = CPUMode::SINGLE;
    let mut cpu_threads         = u8::try_from(std::thread::available_parallelism().unwrap().get()).unwrap();

    let mut show_counter: bool      = false;

    for (index, arg) in collector.iter().enumerate() {
        match arg.as_str() {
            "--help"           => help_section(),
            "-n" | "--numbers" => mode              = Modes::NUMBERS,
            "-g" | "--graphic" => mode              = Modes::GRAPHIC,
            "-m" | "--multi"   => threading_mode    = CPUMode::MULTI,
            "-s" | "--single"  => threading_mode    = CPUMode::SINGLE,
            "-c" | "--counter" => show_counter      = true,
            "-h" | "--threads" => cpu_threads       = collector.get(index + 1)
                                                               .map_or(10,|val| val.parse::<u8>().map_or(10, |cpu_threads| cpu_threads)),
            _    => {  }
        };

        if arg.as_str() == "-t" {
            bench_time = collector.get(index + 1).map_or(10, |val| val.parse::<u64>().map_or(10, |parsed_time| parsed_time));
        }
    }

    cli_wins::main::startup(bench_time, &mode);

    utilities::start_counter();

    match mode {
        Modes::NUMBERS => args.bench_num(bench_time, &threading_mode, cpu_threads, show_counter),
        Modes::GRAPHIC | Modes::NONE => { },
    }
}