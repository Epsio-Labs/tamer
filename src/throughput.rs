use std::fmt::{Display, Formatter};
use std::time::Duration;
use crate::measurement::DurationFormatter;

#[derive(Clone, Debug)]
pub enum Throughput {
    Elements(u64),
}

impl Throughput {
    pub fn value(&self) -> u64 {
        match self {
            Throughput::Elements(e) => *e,
        }
    }

    pub fn units(&self) -> &'static str {
        match self {
            Throughput::Elements(_) => "elements",
        }
    }

    pub fn per_second_string(&self, duration: Duration) -> String {
        match self {
            Throughput::Elements(e) => DurationFormatter::elements_per_second(*e as f64, duration),
        }
    }
}

impl Display for Throughput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Throughput::Elements(e) => write!(f, "{} elements", e),
        }
    }
}
