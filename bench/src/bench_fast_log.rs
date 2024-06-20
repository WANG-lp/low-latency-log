pub mod common;

pub use common::fast_log_bench::bench_fast_log;
use tcmalloc::TCMalloc;

#[global_allocator]
static GLOBAL: TCMalloc = TCMalloc;
fn main() {
    bench_fast_log();
}
