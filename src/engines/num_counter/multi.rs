use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::utilities::delete_last_line;

pub fn benchmark(time: u64, num_threads: u8) {
    let running = Arc::new(AtomicBool::new(true));
    let mut handles = vec![];

    let bench_time = time;

    for _ in 0..num_threads {
        let running_clone = running.clone();
        handles.push(thread::spawn(move || {
            let mut c: u64 = 0;

            while running_clone.load(Ordering::Relaxed) {
                c += 1;
            }

            c
        }));
    }

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(bench_time));
        running.store(false, Ordering::Relaxed);
    });

    let mut results = Vec::new();

    for handle in handles {
        results.push(handle.join().unwrap());
    }

    let total_count: u64 = results.iter().sum();
    let bench_time_ms = bench_time * 1000;
    let result = (total_count + bench_time_ms) / 900_000;

    delete_last_line();
    thread::sleep(Duration::from_secs(1));
    println!("            Finished Benchmark");

    crate::cli_wins::result::result(result);

    // println!("{}", total_count);
}