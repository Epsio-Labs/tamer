use std::time::Duration;
use tamer::bench_info::BenchInfo;
use tamer::expr_to_strings;
use tamer::bencher::Bencher;
use tamer::tamer::Tamer;
use tamer::throughput::Throughput;

async fn sum(a: u64, b: u64) -> u64 {
    a + b
}

fn test_tamer(tamer: &mut Tamer) {
    let g = tamer.benchmark_group("test_tamer");
    g.with_parameters(expr_to_strings!("a", "b"));

    g.bench(BenchInfo::new("async_bench", Throughput::Elements(1))
                      .with_params(expr_to_strings!(1, 2)),
            move |b| b.to_async().bench(move || sum(1, 2)));
    g.bench_with_input(
        BenchInfo::new("async_bench_with_input", Throughput::Elements(1))
                      .with_params(expr_to_strings!(1, 2)),
        move || {
            std::thread::sleep(Duration::from_secs(1));
            (1, 2)
        },
        move |b, i| {
            b.to_async().bench(move || {
                std::thread::sleep(Duration::from_secs(1));
                sum(i.0, i.1)
            })
        });
}

fn main() {
    let mut t = Tamer::default().configure_from_args();
    test_tamer(&mut t);
    t.benchmark();
}