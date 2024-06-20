use low_latency_log::RollingCondition;

use crate::common::{benchmark_latency_func, benchmark_throughput_func};

pub fn bench() {
    println!("======= LLL Benchmark =======");
    let rc = RollingCondition::new().daily();
    let _guard = low_latency_log::Logger::new(rc, "/dev/shm/logbench".to_string(), "log.log".to_string())
        .cpu(1)
        .queue_size(1024 * 1024)
        .background_sleep_time_step_nanos(500)
        .std_log(true)
        // .time_format_str("%H:%M:%S")
        .init()
        .unwrap();

    let fast_log_func = |iter: usize, msg: usize, d: f64| {
        low_latency_log::info!(
            "Logging iteration: {}, message: {}, double: {}",
            iter,
            msg,
            d
        );
        // low_latency_log::info!("hello");
    };
    benchmark_latency_func(1, fast_log_func);
    benchmark_latency_func(4, fast_log_func);
    // benchmark_latency_func(8, fast_log_func);

    benchmark_throughput_func(fast_log_func, || {
        // we can finish the logger mannuly to force flush
        low_latency_log::Logger::finish()
    });
    println!("================================");
}
