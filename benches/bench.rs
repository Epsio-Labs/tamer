use tamer::expr_to_strings;
use tamer::tamer::Tamer;
use tamer::throughput::Throughput;

async fn sum(a: u64, b: u64) -> u64 {
    a + b
}

fn test_tamer(tamer: &mut Tamer) {
    let g = tamer.benchmark_group("test_tamer");
    g.with_parameters(expr_to_strings!("a", "b"));

    g.async_bench_function("async_bench_function".to_string(), Throughput::Elements(1), expr_to_strings!(1, 2), move || sum(1, 2));
}

fn main() {
    let mut t = Tamer::default().configure_from_args();
    test_tamer(&mut t);
    t.benchmark();
}