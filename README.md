# Fastest and low latency logging library for Rust 🪵
### Example
```rust
use fastlog::{info, Level};
use std::fs;

fn main() {
    fastlog::Logger::new()
        .level(Level::Debug)
        .cpu(2)
        .file("my-log-file.log")
        .init()
        .expect("Unable to construct logger");

    for i in 1..1_000_001 {
        info!("number {}", i);
    }
    fastlog::logger().flush();
}
```

fastlog is heavily inspired by `logflume`(https://github.com/SBentley/logflume).

