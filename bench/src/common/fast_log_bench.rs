use fast_log::{Config, FastLogFormat};

use crate::common::{benchmark_latency_func, benchmark_throughput_func};

pub fn bench_fast_log() {
    println!("======= fast_log Benchmark =======");

    fast_log::init(
        Config::new()
            .file("/dev/shm/logbench.test.log")
            .format(FastLogFormat::new())
            .chan_len(Some(1024 * 1024)),
    )
    .unwrap();

    let fast_log_func = |iter: usize, msg: usize, d: f64| {
        log::info!(
            "Logging iteration: {}, message: {}, double: {}",
            iter,
            msg,
            d
        );
    };
    benchmark_latency_func(1, fast_log_func);
    benchmark_latency_func(4, fast_log_func);
    // benchmark_latency_func(8, spdlog_log_func);

    benchmark_throughput_func(fast_log_func, || {
        log::logger().flush();
    });
    println!("================================");
}
