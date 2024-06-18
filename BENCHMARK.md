# Benchmark

## MacBook Pro 16' M3Max 48GB - MacOs Sonoma 14.5 - Rust 1.78.0
- fastlog:
    ```bash
    $ ./target/release/examples/bench_it 
    ======= Fastlog Benchmark =======
    Thread count 1 - Total Messages 2000000 - nanos/msg
    | 50th | 75th | 90th | 95th | 99th | 99.9th | Worst |
    |  56  |  62  |  75  |  93  | 241  |  400   |177806 |

    Thread count 4 - Total Messages 8000000 - nanos/msg
    | 50th | 75th | 90th | 95th | 99th | 99.9th | Worst |
    |  54  |  64  |  79  | 104  | 243  |  437   | 9550  |

    Throughput is 4.217 million msgs/sec average, total time elapsed: 948.514363 ms for 4000000 log messages
    ```

- spdlog-rs:
    ```bash
    $ ./target/release/examples/bench_spdlog
    ======= Spdlog-rs Benchmark =======
    Thread count 1 - Total Messages 2000000 - nanos/msg
    | 50th | 75th | 90th | 95th | 99th | 99.9th | Worst |
    | 250  | 270  | 412  | 429  | 458  |  750   | 4243  |

    Thread count 4 - Total Messages 8000000 - nanos/msg
    | 50th | 75th | 90th | 95th | 99th | 99.9th | Worst |
    | 258  | 291  | 352  | 418  | 639  |  1068  | 29454 |

    Throughput is 2.577 million msgs/sec average, total time elapsed: 1552.470424 ms for 4000000 log messages 

    ================================
    ```

## Tencent Cloud - Intel(R) Xeon(R) Gold 6231C - 4C16G - Rockylinux 9.3 - Linux 5.14.0 - Rust 1.79.0

- fastlog:
    ```bash
    $ ./target/release/examples/bench_it 
    ======= Fastlog Benchmark =======
    Thread count 1 - Total Messages 2000000 - nanos/msg
    | 50th | 75th | 90th | 95th | 99th | 99.9th | Worst |
    | 112  | 123  | 135  | 145  | 188  |  615   | 2223  |

    Thread count 4 - Total Messages 8000000 - nanos/msg
    | 50th | 75th | 90th | 95th | 99th | 99.9th | Worst |
    | 177  | 210  | 231  | 251  | 400  |  1764  |732873 |

    Throughput is 2.609 million msgs/sec average, total time elapsed: 1532.95113 ms for 4000000 log messages

    ================================
    ```

- spdlog-rs:
    ```bash
    $ ./target/release/examples/bench_spdlog
    ======= Spdlog-rs Benchmark =======
    Thread count 1 - Total Messages 2000000 - nanos/msg
    | 50th | 75th | 90th | 95th | 99th | 99.9th | Worst |
    | 592  | 635  | 681  | 726  | 942  |  1652  | 72767 |

    Thread count 4 - Total Messages 8000000 - nanos/msg
    | 50th | 75th | 90th | 95th | 99th | 99.9th | Worst |
    | 637  | 852  | 1998 | 6798 |23751 | 44338  |1076594|

    Throughput is 1.478 million msgs/sec average, total time elapsed: 2705.644195 ms for 4000000 log messages

    ================================
    ```

- quill:

    `MIN_WAIT_DURATION` set to 200, `MAX_WAIT_DURATION` set to 220, output folder set to `/dev/shm/`
    ```bash
    $ ./benchmarks/hot_path_latency/BENCHMARK_quill_hot_path_system_clock
    running for 1 thread(s)
    Thread Count 1 - Total messages 2000000 - Logger: Quill - Benchmark: Hot Path Latency / Nanoseconds
    |  50th | 75th | 90th | 95th | 99th | 99.9th | Worst |
    |  36  |  44  |  53  |  61  |  707  |  1086  |  2648  |

    running for 4 thread(s)
    Thread Count 4 - Total messages 8000000 - Logger: Quill - Benchmark: Hot Path Latency / Nanoseconds
    |  50th | 75th | 90th | 95th | 99th | 99.9th | Worst |
    |  73  |  87  |  109  |  130  |  186  |  3451  |  31143  |

    $ ./benchmarks/backend_throughput/BENCHMARK_quill_backend_throughput
    ...
    Throughput is 2.24 million msgs/sec average, total time elapsed: 1785 ms for 4000000 log messages
    ```