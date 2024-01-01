#![allow(clippy::match_same_arms)]

use std::env;
use crate::engines;
use crate::type_modes::CPUMode;

pub struct Args {
    pub args: Vec<String>,
    pub command: String,
    pub bench_time: u64,
}

impl Args {
    pub const fn new() -> Self {
        Self {
            args: vec![],
            command: String::new(),
            bench_time: 0,
        }
    }

    pub fn collect_bench_time(&mut self) {
        self.bench_time = self.args.get(1).map_or(10, |n| n.parse().unwrap());
    }

    pub fn collector(&mut self) -> Vec<String> {
        let args: Vec<String> = env::args().collect();
        self.args = args.clone();

        args
    }

    pub fn bench_num(&mut self, bench_time: u64, threading_mode: &CPUMode, cpu_threads: u8, show_counter: bool) {
        self.bench_time = bench_time;

        match threading_mode {
            CPUMode::SINGLE | CPUMode::NONE => engines::num_counter::single::benchmark(bench_time, show_counter),
            CPUMode::MULTI => engines::num_counter::multi::benchmark(bench_time, cpu_threads),
        }
    }
}
