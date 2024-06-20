# Benchmark

## MacBook Pro 16' M3Max 48GB - MacOs Sonoma 14.5 - Rust 1.78.0
- low_latency_log:
    ```bash
    $ ./target/release/examples/bench_it 
    ======= LLL Benchmark =======
    Thread count 1 - Total Messages 2000000 - nanos/msg
    | 50th | 75th | 90th | 95th | 99th | 99.9th | Worst |
    |  58  |  66  |  77  |  83  | 216  |  310   | 2372  |

    Thread count 4 - Total Messages 8000000 - nanos/msg
    | 50th | 75th | 90th | 95th | 99th | 99.9th | Worst |
    |  56  |  68  |  89  | 125  | 272  |  489   | 6922  |

    Throughput is 4.563 million msgs/sec average, total time elapsed: 876.626608 ms for 4000000 log messages 

    ================================
    ```
- quill:
    ```bash
    $ ./benchmarks/hot_path_latency/BENCHMARK_quill_hot_path_system_clock
    running for 1 thread(s)
    Thread Count 1 - Total messages 2000000 - Logger: Quill - Benchmark: Hot Path Latency / Nanoseconds
    |  50th | 75th | 90th | 95th | 99th | 99.9th | Worst |
    |  22  |  25  |  31  |  35  |  50  |  206  |  970  |

    running for 4 thread(s)
    Thread Count 4 - Total messages 8000000 - Logger: Quill - Benchmark: Hot Path Latency / Nanoseconds
    |  50th | 75th | 90th | 95th | 99th | 99.9th | Worst |
    |  33  |  33  |  41  |  66  |  166  |  783  |  20908  |

    $ ./benchmarks/backend_throughput/BENCHMARK_quill_backend_throughput
    ...
    Throughput is 2.88 million msgs/sec average, total time elapsed: 1390 ms for 4000000 log messages
    ```

- fast_log:
    ```bash
    $ ./target/release/bench_fast_log
    ======= fast_log Benchmark =======
    Thread count 1 - Total Messages 2000000 - nanos/msg
    | 50th | 75th | 90th | 95th | 99th | 99.9th | Worst |
    | 275  | 297  | 416  | 450  | 508  |  827   | 2075  |

    Thread count 4 - Total Messages 8000000 - nanos/msg
    | 50th | 75th | 90th | 95th | 99th | 99.9th | Worst |
    | 277  | 310  | 427  | 493  | 741  |  1093  | 18070 |

    Throughput is 1.741 million msgs/sec average, total time elapsed: 2298.054076 ms for 4000000 log messages 

    ================================
    ```

- spdlog-rs:
    ```bash
    $ ./target/release/examples/bench_spdlog
    ======= Spdlog-rs Benchmark =======
    Thread count 1 - Total Messages 2000000 - nanos/msg
    | 50th | 75th | 90th | 95th | 99th | 99.9th | Worst |
    | 252  | 270  | 404  | 427  | 456  |  735   | 8097  |

    Thread count 4 - Total Messages 8000000 - nanos/msg
    | 50th | 75th | 90th | 95th | 99th | 99.9th | Worst |
    | 266  | 310  | 385  | 464  | 775  |  1225  | 15581 |

    Throughput is 2.605 million msgs/sec average, total time elapsed: 1535.519136 ms for 4000000 log messages 

    ================================
    ```

- ftlog:
    ```bash
    $ ./target/release/bench_ftlog
    ======= ftlog Benchmark =======
    Thread count 1 - Total Messages 2000000 - nanos/msg
    | 50th | 75th | 90th | 95th | 99th | 99.9th | Worst |
    | 341  | 362  | 483  | 516  | 568  |  1089  | 13370 |

    Thread count 4 - Total Messages 8000000 - nanos/msg
    | 50th | 75th | 90th | 95th | 99th | 99.9th | Worst |
    | 333  | 379  | 479  | 587  | 966  |  1533  |118018 |

    Throughput is 1.688 million msgs/sec average, total time elapsed: 2369.064489 ms for 4000000 log messages 

    ================================
    ```



## Tencent Cloud - Intel(R) Xeon(R) Gold 6231C - 4C16G - Rockylinux 9.3 - Linux 5.14.0 - Rust 1.79.0

- low_latency_log:
    ```bash
    $ ./target/release/examples/bench_it 
    ======= LLL Benchmark =======
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