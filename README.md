# Fast and Low Latency Logging Library for Rust

## Introduction
`low-latency-log` is a high-performance and low-latency Rust logging library.

## Features
* **Very Low Latency**: `low-latency-log` is designed with performance in mind, utilizing techniques such as minimizing the size of critical data structures, avoiding locks on critical paths, and caching formatted strings.
* **Async Logging**: `low-latency-log` offloads all heavy logging operations (such as formatting, time conversion, etc.) to independent threads, ensuring the calling thread is not blocked.

## Benchmark
`low-latency-log` offers comparable p999 latency to [`quill`](https://github.com/odygrd/quill) and leads in throughput among `quil`, `spdlog-rs`, `ftlog`, and `fast_log`.

For more details, please refer to the [Benchmark](./BENCHMARK.md).

To build the benchmark binaries, run: 
```sh
cargo b -r -p bench
```

## Usage example
```rust
use low_latency_log::{info, Level};
use std::fs;

fn main() {
    let rc = RollingCondition::new().daily();
    // Remember to keep the following guard, otherwise the global logger stops immediately when the guard auto-drops
    let _guard = low_latency_log::Logger::new(rc, "/dev/shm".to_string(), "log.log".to_string())
        .cpu(1)
        .init()
        .unwrap();

    for i in 1..1_000_001 {
        info!("number {}", i);
    }

    // _guard auto-dropped and log flushed
}
```

## TODOs
The following optimizations are in progress:
- Optimize std `format!`.
- Improve `ufmt` to provide more types of formatting support (e.g., floating-point types).
- Support custom format types, as currently `low_latency_log` outputs fixed time and log formats.
- Optimize performance when using the `log` crate.

## `low_latency_log` is heavily inspired by the following projects

* [`logflume`](https://github.com/SBentley/logflume)
* [`quill`](https://github.com/odygrd/quill)
* [`spdlog-rs`](https://github.com/SpriteOvO/spdlog-rs)
* [`rolling-file-rs`](https://github.com/Axcient/rolling-file-rs)

## License
This project is licensed under the Apache License.

Some code comes from the `logflume` project. Please refer to [LICENSE-LOGFLUME](./LICENSE-LOGFLUME) for more information.