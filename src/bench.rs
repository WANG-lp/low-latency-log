use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

use fastlog::{info, Level};

const ITERATIONS: usize = 100000;
const MESSAGES_PER_ITERATION: usize = 20;
const MIN_WAIT_DURATION_NS: usize = 200_000;
const MAX_WAIT_DURATION_NS: usize = 220_000;

fn bench_mark_func(thread_count: usize) {
    let mut latencies = Vec::with_capacity(thread_count * ITERATIONS);

    let mut thread_handlers = Vec::with_capacity(thread_count);

    for _ in 0..thread_count {
        thread_handlers.push(std::thread::spawn(|| {
            let mut latencies_per_thread = Vec::with_capacity(ITERATIONS);
            let mut aux: u32 = 0;
            for iter in 0..ITERATIONS {
                // let start = unsafe { core::arch::x86_64::__rdtscp(&mut aux) };
                let start = std::time::Instant::now();
                for _ in 0..MESSAGES_PER_ITERATION {
                    info!("hello {}", 10);
                }
                // let end = unsafe { core::arch::x86_64::__rdtscp(&mut aux) };
                latencies_per_thread
                    .push((start.elapsed().as_nanos() as u64) / MESSAGES_PER_ITERATION as u64);
                let now = std::time::Instant::now();
                let end_ns = fastrand::usize(MIN_WAIT_DURATION_NS..MAX_WAIT_DURATION_NS);
                while (now.elapsed().as_nanos() as usize) < end_ns {
                    // wait
                }
            }
            latencies_per_thread
        }));
    }

    for handler in thread_handlers {
        let latencies_per_thread = handler.join().unwrap();
        latencies.extend(latencies_per_thread);
    }

    latencies.sort();

    println!(
        "Thread count {} - Total Messages {}",
        thread_count,
        thread_count * ITERATIONS * MESSAGES_PER_ITERATION
    );
    println!("  |  50th | 75th | 90th | 95th | 99th | 99.9th | Worst |");
    println!(
        "  |  {}  |  {}  |  {}  |  {}   |  {}   |  {}   |  {}  |",
        latencies[latencies.len() / 2],
        latencies[latencies.len() * 3 / 4],
        latencies[latencies.len() * 9 / 10],
        latencies[latencies.len() * 19 / 20],
        latencies[latencies.len() * 99 / 100],
        latencies[latencies.len() * 999 / 1000],
        latencies[latencies.len() - 1]
    );
}

fn main() {
    if Path::new("/dev/shm/test.log").exists() {
        fs::remove_file("/dev/shm/test.log").expect("Cannot delete benchmark log file.");
    }
    fastlog::Logger::new()
        .level(Level::Info)
        .cpu(1)
        .file("/dev/shm/test.log")
        .sleep_duration_millis(50)
        .buffer_size(131072 * 2)
        .init()
        .expect("Unable to construct logger");

    bench_mark_func(1);
    bench_mark_func(4);
}
