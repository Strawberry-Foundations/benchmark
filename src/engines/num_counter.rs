use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::thread::sleep;

use crate::colors::{C_RESET, YELLOW};
use crate::utilities::delete_last_line;

pub fn benchmark(time: u64, showcounter: bool) {
    if showcounter { 
        println!("{YELLOW}WARNING: You have enabled 'showcounter', this currently drags down the score heavily, used for testing.{C_RESET}");
        sleep(Duration::from_secs(5));
    
    }
    
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();

    let mut x: u64 = 0;

    let bench_time = time;

    thread::spawn(move || {
        sleep(Duration::from_secs(bench_time));
        running_clone.store(false, Ordering::Relaxed);
    });

    if showcounter { println!("{YELLOW}WARNING: You have enabled 'showcounter', this currently drags down the score heavily, used for testing.{C_RESET}") }

    while running.load(Ordering::Relaxed) {
        x += 1;
        if showcounter && x % 10000 == 0 {
            delete_last_line();
            println!("{x}");
        }
    }

    let bench_time_ms = bench_time * 1000;
    let result = (x + bench_time_ms) / 900_000;

    delete_last_line();
    sleep(Duration::from_secs(1));
    println!("            Finished Benchmark");

    crate::cli_wins::result(result);
}
