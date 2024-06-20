use fastlog::RollingCondition;

use crate::common::{benchmark_latency_func, benchmark_throughput_func};

pub fn bench_fastlog() {
    println!("======= Fastlog Benchmark =======");
    let rc = RollingCondition::new().daily();
    fastlog::Logger::new(rc, "/dev/shm/logbench".to_string(), "log.log".to_string())
        .cpu(1)
        .queue_size(1024 * 1024)
        .background_sleep_time_step_nanos(500)
        .init()
        .unwrap();

    let fast_log_func = |iter: usize, msg: usize, d: f64| {
        fastlog::info!(
            "Logging iteration: {}, message: {}, double: {}",
            iter,
            msg,
            d
        );
        // fastlog::info!("hello");
    };
    benchmark_latency_func(1, fast_log_func);
    benchmark_latency_func(4, fast_log_func);
    // benchmark_latency_func(8, fast_log_func);

    benchmark_throughput_func(fast_log_func, || fastlog::Logger::finish());
    println!("================================");
}
