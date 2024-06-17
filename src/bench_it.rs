use fastlog::{info, LogLevel, RollingCondition};

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
            for iter in 0..ITERATIONS {
                let d = iter as f64 + (0.1 * iter as f64);

                let start = std::time::Instant::now();
                for m_id in 0..MESSAGES_PER_ITERATION {
                    info!(
                        "Logging iteration: {}, message: {}, double: {}",
                        iter, m_id, d
                    );
                    // info!("hello");
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
    let rc = RollingCondition::new().daily();
    fastlog::Logger::new(
        131072,
        rc,
        "/dev/shm".to_string(),
        "log.log".to_string(),
        30,
        LogLevel::Info,
        Some(1),
    )
    .init()
    .unwrap();

    bench_mark_func(1);
    bench_mark_func(4);
}
