use crate::type_modes::{CPUMode, Modes};

pub struct Benchmark {
    pub mode: Modes,
    pub benchmark_duration: u64,
    pub num_counter: NumCounter
}

pub struct NumCounter {
    pub threading_mode: CPUMode,
    pub threads: u8,
    pub counter: bool,
}

impl Benchmark {
    pub fn new() -> Self {
        Self {
            mode: Modes::NUMBERS,
            benchmark_duration: 10,
            num_counter: NumCounter {
                threading_mode: CPUMode::SINGLE,
                threads: u8::try_from(std::thread::available_parallelism().unwrap().get()).unwrap(),
                counter: false,
            },
        }
    }
}