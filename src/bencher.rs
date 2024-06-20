use std::future::Future;
use std::time::Duration;
use tokio::runtime::Runtime;
use crate::bench::BenchSummary;
use crate::bench_info::BenchInfo;

pub struct AsyncBencher {}

impl AsyncBencher {
    pub fn new() -> Self {
        AsyncBencher {}
    }
}

impl<FR, R: Future<Output = FR>> Bencher<R> for AsyncBencher {
    fn bench<F: FnOnce() -> R + 'static>(&mut self, f: F) -> Duration {
        let runner = Runtime::new().unwrap();

        runner.block_on(async {
            let start = std::time::Instant::now();
            let r = f().await;
            drop(r); // include drop time in the benchmark
            start.elapsed()
        })
    }
}

pub struct SyncBencher {}

impl SyncBencher {
    pub fn new() -> Self {
        SyncBencher {}
    }

    pub fn to_async(&self) -> AsyncBencher {
        AsyncBencher::new()
    }
}

impl<R> Bencher<R> for SyncBencher {
    fn bench<F: FnOnce() -> R + 'static>(&mut self, f: F) -> Duration {
        let start = std::time::Instant::now();
        let r = f();
        drop(r); // include drop time in the benchmark
        start.elapsed()
    }
}

pub trait Bencher<R> {
    fn bench<F: FnOnce() -> R + 'static>(&mut self, f: F) -> Duration;
}
