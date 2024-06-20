pub mod common;

pub use common::ftlog_bench::bench_ftlog;
use tcmalloc::TCMalloc;

#[global_allocator]
static GLOBAL: TCMalloc = TCMalloc;
fn main() {
    bench_ftlog();
}
