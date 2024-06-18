use std::future::Future;
use std::path::PathBuf;
use crate::bench::Bench;
use crate::throughput::Throughput;
use tokio::runtime::Runtime;

pub struct Group {
    name: String,
    output_dir: Option<PathBuf>,
    parameters_names: Vec<String>,
    benches: Vec<Bench>,
}

impl Group {
    pub(crate) fn new(name: String, output_dir: Option<PathBuf>) -> Group {
        Group {
            name,
            output_dir,
            parameters_names: vec![],
            benches: Vec::new(),
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

    pub fn report(&self) {
        if let Some(p) = &self.output_dir {
            let output_dir = p.join("bench").join(self.name.clone());
            let output_path = output_dir.join("report.csv");
            println!("Writing report to {:?}", output_path);
            std::fs::create_dir_all(&output_dir).unwrap();
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

#[macro_export]
macro_rules! expr_to_strings {
    ( $( $expr:expr ),* $(,)? ) => {
        vec![
            $(
                format!("{}", $expr)
            ),*
        ]
    };
}