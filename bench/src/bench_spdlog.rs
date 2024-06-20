pub mod common;

pub use common::spdlog_bench::bench_spdlog;
use tcmalloc::TCMalloc;

#[global_allocator]
static GLOBAL: TCMalloc = TCMalloc;
fn main() {
    bench_spdlog();
}
