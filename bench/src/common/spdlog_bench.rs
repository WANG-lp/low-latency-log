use std::{path::PathBuf, sync::Arc};

use crate::common::{benchmark_latency_func, benchmark_throughput_func};

static GLOBAL_SPDLOG_LOGGER: once_cell::sync::OnceCell<Arc<spdlog::Logger>> =
    once_cell::sync::OnceCell::new();

pub fn bench_spdlog() {
    println!("======= Spdlog-rs Benchmark =======");

    const LOG_FILE: &str = "/dev/shm/logbench/spdlog_async_file_sink.log";

    let path: PathBuf = LOG_FILE.into();
    let file_sink: Arc<spdlog::sink::FileSink> = Arc::new(
        spdlog::sink::FileSink::builder()
            .path(path)
            .truncate(true)
            .build()
            .unwrap(),
    );

    let thread_pool = Arc::new(spdlog::ThreadPool::builder().build().unwrap());
    let async_sink = Arc::new(
        spdlog::sink::AsyncPoolSink::builder()
            .thread_pool(thread_pool)
            .overflow_policy(spdlog::sink::OverflowPolicy::Block)
            .sink(file_sink)
            .build()
            .unwrap(),
    );

    let logger: Arc<spdlog::Logger> =
        Arc::new(spdlog::Logger::builder().sink(async_sink).build().unwrap());

    spdlog::set_default_logger(logger.clone());
    let _ = GLOBAL_SPDLOG_LOGGER.set(logger);

    let new_formatter: Box<spdlog::formatter::PatternFormatter<_>> = Box::new(
        spdlog::formatter::PatternFormatter::new(spdlog::formatter::pattern!(
            "{hour}:{minute}:{second}:{nanosecond} {^[{tid}]} {file}:{line} {level} {payload}{eol}"
        )),
    );
    for sink in spdlog::default_logger().sinks() {
        sink.set_formatter(new_formatter.clone())
    }

    let spdlog_log_func = |iter: usize, msg: usize, d: f64| {
        spdlog::info!(
            "Logging iteration: {}, message: {}, double: {}",
            iter,
            msg,
            d
        );
        // spdlog::info!("hello");
    };
    benchmark_latency_func(1, spdlog_log_func);
    benchmark_latency_func(4, spdlog_log_func);
    // benchmark_latency_func(8, spdlog_log_func);

    benchmark_throughput_func(spdlog_log_func, || {
        GLOBAL_SPDLOG_LOGGER.get().unwrap().flush();
    });
    println!("================================");
}
