pub mod common;

pub use common::low_latency_log_bench::bench;
use tcmalloc::TCMalloc;
#[global_allocator]
static GLOBAL: TCMalloc = TCMalloc;
fn main() {
    bench();
}
