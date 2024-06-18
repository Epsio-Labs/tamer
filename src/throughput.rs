use std::fmt::{Display, Formatter};
use std::time::Duration;

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

    pub fn per_second(&self, duration: Duration) -> f64 {
        self.value() as f64 / duration.as_secs_f64()
    }

    pub fn per_second_string(&self, duration: Duration) -> String {
        format!("{:.2} {}/s", self.per_second(duration), self.units())
    }
}

impl Display for Throughput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Throughput::Elements(e) => write!(f, "{} elements", e),
        }
    }
}
