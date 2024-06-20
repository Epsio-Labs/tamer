use std::fmt::{Display, Formatter};
use std::time::Duration;
use crate::bench_info::BenchInfo;

/// Bench results
pub struct BenchSummary {
    pub(crate) bench_info: BenchInfo,
    pub(crate) elapsed_time: Duration,
}

impl BenchSummary {
    pub fn new(bench_info: BenchInfo, elapsed_time: Duration) -> Self {
        BenchSummary {
            bench_info,
            elapsed_time,
        }
    }
}

pub(crate) struct BenchTask {
    pub(crate) bench_info: BenchInfo,
    pub(crate) benchmark_function: Box<dyn FnOnce() -> BenchSummary>
}

impl Display for BenchSummary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Duration: {}ms\nThroughout: {}",
            self.elapsed_time.as_millis(),
            self.bench_info.throughput.per_second_string(self.elapsed_time),
        )
    }
}
