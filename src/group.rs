use std::future::Future;
use std::path::PathBuf;
use chrono::Utc;
use crate::bench::{Bench, Bencher};
use crate::throughput::Throughput;
use tokio::runtime::Runtime;
use crate::tamer::BenchmarkFilter;

pub struct Group {
    pub(crate) name: String,
    output_dir: Option<PathBuf>,
    parameters_names: Vec<String>,
    benchers: Vec<Bencher>,
    benches: Vec<Bench>,
    did_report: bool,
}

impl Group {
    pub(crate) fn new(name: String, mut output_dir: Option<PathBuf>) -> Group {
        output_dir = output_dir.map(|p| p.join(&name));
        Group {
            name,
            output_dir,
            parameters_names: vec![],
            benchers: Vec::new(),
            benches: Vec::new(),
            did_report: false,
        }
    }

    pub fn with_parameters(&mut self, parameters_names: Vec<String>) -> &mut Self {
        self.parameters_names = parameters_names;
        self
    }

    pub fn async_bench<O, R: Future<Output = O>, F: FnOnce() -> R + 'static>(
        &mut self,
        id: impl ToString,
        throughput: Throughput,
        params: Vec<String>,
        f: F,
    ) {
        let id = id.to_string();
        assert_eq!(params.len(), self.parameters_names.len());
        let bencher = Bencher {
            id: id.clone(),
            benchmark_function: Box::new(move || {
                let runner = Runtime::new().unwrap();

                let bench = runner.block_on(async {
                    let start = std::time::Instant::now();
                    let r = f().await;
                    drop(r); // include drop time in the benchmark
                    let elapsed_time = start.elapsed();
                    Bench {
                        id,
                        throughput,
                        elapsed_time,
                        params,
                    }
                });
                bench
            }),
        };
        self.benchers.push(bencher);
    }

    pub fn async_bench_with_input<I, FI: FnOnce() -> I + 'static, O, R: Future<Output = O>, F: FnOnce(I) -> R + 'static>(
        &mut self,
        id: impl ToString,
        input_function: FI,
        throughput: Throughput,
        params: Vec<String>,
        f: F,
    ) {
        let id = id.to_string();
        assert_eq!(params.len(), self.parameters_names.len());
        let bencher = Bencher {
            id: id.clone(),
            benchmark_function: Box::new(move || {
                let input = (input_function)();
                let runner = Runtime::new().unwrap();

                let bench = runner.block_on(async {
                    let start = std::time::Instant::now();
                    let r = f(input).await;
                    drop(r); // include drop time in the benchmark
                    let elapsed_time = start.elapsed();
                    Bench {
                        id,
                        throughput,
                        elapsed_time,
                        params,
                    }
                });
                bench
            }),
        };
        self.benchers.push(bencher);
    }

    pub(crate) fn benchmark(&mut self, filter: &BenchmarkFilter) {
        let benchers = std::mem::replace(&mut self.benchers, Vec::new());
        let filtered_benchers: Vec<_> = benchers.into_iter().filter(|b| filter.filter_matches(&format!("{}/{}", self.name, b.id))).collect();
        if filtered_benchers.is_empty() {
            return;
        }
        println!("Running group {}:", self.name);
        for bencher in filtered_benchers {
            println!("\t{}:", bencher.id);
            let bench = (bencher.benchmark_function)();
            let bench_display = format!("{}", bench);
            let bench_display = bench_display.replace("\n", "\n\t\t");
            println!("\t\t{}", bench_display);
            self.benches.push(bench);
        }
    }

    pub fn report(&mut self) {
        if self.did_report || self.benches.is_empty() {
            return;
        }
        if let Some(p) = &self.output_dir {
            let formatted_datetime = Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
            let filename = format!("report_{}.csv", formatted_datetime);
            let output_path = p.join(filename);
            println!("Writing report to {:?}", output_path);
            std::fs::create_dir_all(p).unwrap();
            let mut writer = csv::Writer::from_path(output_path).unwrap();

            let mut headers = vec!["id".to_string(), "duration(ms)".to_string(), "throughput".to_string()];
            for p in self.parameters_names.iter() {
                headers.push(p.clone());
            }

            writer.write_record(&headers).unwrap();
            for bench in self.benches.iter() {
                let mut values = vec![
                    bench.id.clone(),
                    bench.elapsed_time.as_millis().to_string(),
                    bench.throughput.value().to_string(),
                ];
                for p in bench.params.clone().into_iter() {
                    values.push(p);
                }
                writer
                    .write_record(&values)
                    .unwrap();
            }
            writer.flush().unwrap();
            self.did_report = true;
        } else {
            println!("CARGO_TARGET_DIR not set, skipping report writing");
        }
    }
}

impl Drop for Group {
    fn drop(&mut self) {
        self.report();
    }
}