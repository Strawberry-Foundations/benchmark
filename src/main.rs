use std::{env, thread};
use std::time::Duration;

fn benchmark(time: u64) {
    let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    let running_clone = running.clone();

    let mut x: u64 = 0;

    let bench_time = time;

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(bench_time));
        running_clone.store(false, std::sync::atomic::Ordering::Relaxed);
    });

    while running.load(std::sync::atomic::Ordering::Relaxed) {
        x += 1;
    }

    let bench_time_ms = bench_time * 1000;
    let result = (x + bench_time_ms) / 900000;

    println!("Eingestelle Benchmark-Zeit: {}s ({}ms)", bench_time, bench_time_ms);
    println!("Der Counter hat bis {} gezählt", x);
    println!("Ergebnis: {result}");
}

fn main() {
    println!("-- Boolean's Benchmarking Tool v1.0.0 --");

    let args: Vec<String> = env::args().collect();
    let bench_time: u64 = if args.len() < 2 {
        10
    }
    else {
        args[1].parse().unwrap()
    };

    benchmark(bench_time);
}