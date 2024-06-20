use ftlog::{
    appender::{Duration, FileAppender, Period},
    FtLogFormatter, LevelFilter,
};

use crate::common::{benchmark_latency_func, benchmark_throughput_func};

pub fn bench_ftlog() {
    println!("======= ftlog Benchmark =======");

    let time_format =
        time::format_description::parse_owned::<1>("[hour]:[minute]:[second].[subsecond digits:9]")
            .unwrap();
    let _guard = ftlog::builder()
        .max_log_level(LevelFilter::Info)
        .time_format(time_format)
        .bounded(1024 * 1024, false)
        .root(
            FileAppender::builder()
                .path("/dev/shm/ftlog.log")
                .rotate(Period::Day)
                .expire(Duration::days(31))
                .build(),
        )
        .format(FtLogFormatter)
        .fixed_timezone(time::UtcOffset::current_local_offset().unwrap())
        .try_init()
        .expect("logger build or set failed");

    let ft_log_func = |iter: usize, msg: usize, d: f64| {
        ftlog::info!(
            "Logging iteration: {}, message: {}, double: {}",
            iter,
            msg,
            d
        );
        // ftlog::info!("hello");
    };
    benchmark_latency_func(1, ft_log_func);
    benchmark_latency_func(4, ft_log_func);
    // benchmark_latency_func(8, spdlog_log_func);

    benchmark_throughput_func(ft_log_func, || {
        log::logger().flush();
    });
    println!("================================");
}
