use crate::throughput::Throughput;

#[derive(Clone)]
pub struct BenchInfo {
    pub(crate) id: String,
    pub(crate) throughput: Throughput,
    pub(crate) params: Vec<String>,
}

impl BenchInfo {
    pub fn new(id: impl ToString, throughput: Throughput) -> Self {
        BenchInfo {
            id: id.to_string(),
            throughput,
            params: Vec::new(),
        }
    }

    pub fn with_params(mut self, params: Vec<String>) -> Self {
        self.params = params;
        self
    }
}
