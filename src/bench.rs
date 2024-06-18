use std::fmt::{Display, Formatter};
use std::time::Duration;
use crate::throughput::Throughput;

pub struct Bench {
    pub(crate) id: String,
    pub(crate) throughput: Throughput,
    pub(crate) elapsed_time: Duration,
}

impl Display for Bench {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:\n\tDuration: {}ms\n\tThroughout: {}",
            self.id,
            self.elapsed_time.as_millis(),
            self.throughput.per_second_string(self.elapsed_time),
        )
    }
}
