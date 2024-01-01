use std::env;

pub struct Args {
    pub args: Vec<String>,
    pub bench_time: u64,
}

impl Args {
    pub const fn new() -> Self {
        Self {
            args: vec![],
            bench_time: 0,
        }
    }

    pub fn collect_bench_time(&mut self) {
        self.bench_time = self.args.get(1).map_or(10, |n| n.parse().unwrap());
    }

    pub fn collector(&mut self) -> Vec<String> {
        let args: Vec<String> = env::args().collect();
        self.args = args.clone();

        self.collect_bench_time();

        args
    }
}
