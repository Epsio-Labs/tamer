use std::future::Future;
use std::path::PathBuf;
use std::time::Duration;
use chrono::Utc;
use crate::bench::{BenchSummary, BenchTask};
use crate::bench_info::BenchInfo;
use crate::bencher::{AsyncBencher, Bencher, SyncBencher};
use crate::tamer::BenchmarkFilter;

pub struct Group {
    pub(crate) name: String,
    output_dir: Option<PathBuf>,
    parameters_names: Vec<String>,
    benchers: Vec<BenchTask>,
    benches: Vec<BenchSummary>,
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

    pub fn bench<F: FnOnce(SyncBencher) -> Duration + 'static>(
        &mut self,
        bench_info: BenchInfo,
        f: F,
    ) {
        assert_eq!(bench_info.params.len(), self.parameters_names.len());
        let bench_task = BenchTask {
            bench_info: bench_info.clone(),
            benchmark_function: Box::new(move || {
                BenchSummary::new(bench_info, f(SyncBencher::new()))
            })
        };
        self.benchers.push(bench_task);
    }

    pub fn bench_with_input<I: 'static, FI: FnOnce() -> I + 'static, F: FnOnce(SyncBencher, I) -> Duration + 'static>(
        &mut self,
        bench_info: BenchInfo,
        input_function: FI,
        f: F,
    ) {
        assert_eq!(bench_info.params.len(), self.parameters_names.len());
        self.benchers.push(BenchTask {
            bench_info: bench_info.clone(),
            benchmark_function: Box::new(move || {
                let input = (input_function)();
                BenchSummary::new(bench_info, f(SyncBencher::new(), input))
            })
        });
    }

    pub(crate) fn benchmark(&mut self, filter: &BenchmarkFilter) {
        let benchers = std::mem::replace(&mut self.benchers, Vec::new());
        let filtered_benchers: Vec<_> = benchers.into_iter().filter(|b| filter.filter_matches(&format!("{}/{}", self.name, b.bench_info.id))).collect();
        if filtered_benchers.is_empty() {
            return;
        }
        println!("Running group {}:", self.name);
        for bencher in filtered_benchers {
            println!("\t{}:", bencher.bench_info.id);
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
                    bench.bench_info.id.clone(),
                    bench.elapsed_time.as_millis().to_string(),
                    bench.bench_info.throughput.value().to_string(),
                ];
                for p in bench.bench_info.params.clone().into_iter() {
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