use std::future::Future;
use std::path::PathBuf;
use chrono::Utc;
use crate::bench::Bench;
use crate::throughput::Throughput;
use tokio::runtime::Runtime;

pub struct Group {
    name: String,
    output_dir: Option<PathBuf>,
    parameters_names: Vec<String>,
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
            benches: Vec::new(),
            did_report: false,
        }
    }

    pub fn with_parameters(&mut self, parameters_names: Vec<String>) {
        self.parameters_names = parameters_names;
    }

    pub fn async_bench_function<O, R: Future<Output = O>, F: FnOnce() -> R>(
        &mut self,
        id: String,
        throughput: Throughput,
        params: Vec<String>,
        f: F,
    ) {
        assert_eq!(params.len(), self.parameters_names.len());
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
        println!("{}", bench);
        self.benches.push(bench);
    }

    pub fn report(&mut self) {
        if self.did_report {
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