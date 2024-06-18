use bench::fastlog_bench::bench_fastlog;
use tcmalloc::TCMalloc;

#[global_allocator]
static GLOBAL: TCMalloc = TCMalloc;
fn main() {
    bench_fastlog();
}
