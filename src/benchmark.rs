use std::thread;
use std::time::Duration;

pub(crate) fn benchmark(time: u64) {
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

    crate::cli_wins::result(10000);
}
