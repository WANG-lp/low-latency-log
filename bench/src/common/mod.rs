const ITERATIONS: usize = 100000;
const MESSAGES_PER_ITERATION: usize = 20;
const MIN_WAIT_DURATION_NS: usize = 200_000;
const MAX_WAIT_DURATION_NS: usize = 220_000;
const THROUGHPUT_ITERATION: usize = 4_000_000;

pub mod fastlog_bench;
pub mod spdlog_bench;

fn benchmark_latency_func(thread_count: usize, log_func: fn(usize, usize, f64)) {
    let mut latencies = Vec::with_capacity(thread_count * ITERATIONS);

    let mut thread_handlers = Vec::with_capacity(thread_count);

    for _ in 0..thread_count {
        thread_handlers.push(std::thread::spawn(move || {
            let mut latencies_per_thread = Vec::with_capacity(ITERATIONS);
            for iter in 0..ITERATIONS {
                let d = iter as f64 + (0.1 * iter as f64);

                let start = quanta::Instant::now();
                for m_id in 0..MESSAGES_PER_ITERATION {
                    log_func(iter, m_id, d);
                }
                latencies_per_thread
                    .push((start.elapsed().as_nanos() as u64) / MESSAGES_PER_ITERATION as u64);
                let now = std::time::Instant::now();
                let end_ns = fastrand::usize(MIN_WAIT_DURATION_NS..MAX_WAIT_DURATION_NS);
                while (now.elapsed().as_nanos() as usize) < end_ns {}
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
        "Thread count {} - Total Messages {} - nanos/msg",
        thread_count,
        thread_count * ITERATIONS * MESSAGES_PER_ITERATION
    );
    println!("  | 50th | 75th | 90th | 95th | 99th | 99.9th | Worst |");
    println!(
        "  |{:^6}|{:^6}|{:^6}|{:^6}|{:^6}|{:^8}|{:^7}|\n",
        latencies[latencies.len() / 2],
        latencies[latencies.len() * 3 / 4],
        latencies[latencies.len() * 9 / 10],
        latencies[latencies.len() * 19 / 20],
        latencies[latencies.len() * 99 / 100],
        latencies[latencies.len() * 999 / 1000],
        latencies[latencies.len() - 1]
    );
}

fn benchmark_throughput_func(log_func: fn(usize, usize, f64), flush_func: fn()) {
    let start = quanta::Instant::now();
    for iter in 0..THROUGHPUT_ITERATION {
        let d = iter as f64 / 2.0;
        log_func(iter, iter * 2, d);
    }
    flush_func();
    let elapsed = start.elapsed().as_nanos() as f64;
    println!(
        "Throughput is {:.3} million msgs/sec average, total time elapsed: {} ms for {} log messages \n",
        THROUGHPUT_ITERATION as f64/1_000_000.0 / (elapsed as f64/ 1_000_000_000.0),
        elapsed / 1_000_000.0,
        THROUGHPUT_ITERATION
    );
}
