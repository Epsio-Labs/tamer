extern crate tamer;

use std::env::temp_dir;
use tamer::expr_to_strings;
use tamer::tamer::Tamer;
use tamer::throughput::Throughput;

async fn sum(a: u64, b: u64) -> u64 {
    a + b
}

#[test]
fn test_tamer() {
    let dir = temp_dir();
    let mut tamer = Tamer::new(Some(dir));
    let g = tamer.benchmark_group("test_tamer");
    g.with_parameters(expr_to_strings!("a", "b"));

    g.async_bench_function("test_tamer".to_string(), Throughput::Elements(1), expr_to_strings!(1, 2), move || sum(1, 2));
}