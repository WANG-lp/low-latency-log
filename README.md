# Fast and Low Latency Logging Library for Rust 🪵

## Introduction
Fastlog is a high-performance and low-latency Rust logging library.

## Features
* **Low Latency**: Fastlog is designed and coded with performance factors in mind, such as limiting the size of critical data structures, avoiding any locks on critical paths, and caching formatted strings.
* **Async Logging**: Fastlog offloads all relatively heavy logging operations (such as formatting, time conversion, etc.) to independent threads, ensuring the calling thread is not blocked.
* **Compile-time Selection of Maximum Log Level**: Fastlog allows the maximum log level to be selected at compile-time, avoiding runtime checks.

## Benchmark
please refer [Benchmark](./BENCHMARK.md)

## Usage Example
```rust
use fastlog::{info, Level};
use std::fs;

fn main() {
    let rc = RollingCondition::new().daily();
    fastlog::Logger::new(rc, "/dev/shm".to_string(), "log.log".to_string())
        .cpu(1)
        .init()
        .unwrap();

    for i in 1..1_000_001 {
        info!("number {}", i);
    }
    fastlog::Logger::finish();
}
```

## TODOs
The following optimizations are in progress:
- Optimize `format!`, as `format!` in Rust has performance limitations.
- Improve `ufmt` to provide more types of formatting support (e.g., floating-point types).
- Support custom format types, as currently fastlog outputs fixed time and log formats.

## Fastlog is Heavily Inspired by the Following Projects

* [`logflume`](https://github.com/SBentley/logflume)
* [`quill`](https://github.com/odygrd/quill)
* [`spdlog-rs`](https://github.com/SpriteOvO/spdlog-rs)
* [`rolling-file-rs`](https://github.com/Axcient/rolling-file-rs)

## License
This project is under the Apache license.

Some code comes from the `logflume` project. Please refer to [LICENSE-LOGFLUME](./LICENSE-LOGFLUME) for more information.