use std::future::Future;
use std::path::PathBuf;
use crate::bench::Bench;
use crate::throughput::Throughput;
use tokio::runtime::Runtime;

pub struct Group {
    name: String,
    output_dir: Option<PathBuf>,
    benches: Vec<Bench>,
}

impl Group {
    pub(crate) fn new(name: String, output_dir: Option<PathBuf>) -> Group {
        Group {
            name,
            output_dir,
            benches: Vec::new(),
        }
    }

    pub fn async_bench_function<O, R: Future<Output = O>, F: FnOnce() -> R>(
        &mut self,
        id: String,
        throughput: Throughput,
        f: F,
    ) {
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
            }
        });
        println!("{}", bench);
        self.benches.push(bench);
    }

    /// Finish the group and write the report to the file system
    pub fn finish(&self) {
        if let Some(p) = &self.output_dir {
            let output_dir = p.join("bench").join(self.name.clone());
            let output_path = output_dir.join("report.csv");
            println!("Writing report to {:?}", output_path);
            std::fs::create_dir_all(&output_dir).unwrap();
            let mut writer = csv::Writer::from_path(output_path).unwrap();
            writer
                .write_record(&["id", "duration", "throughput"])
                .unwrap();
            for bench in self.benches.iter() {
                writer
                    .write_record(&[
                        bench.id.clone(),
                        bench.elapsed_time.as_millis().to_string(),
                        bench.throughput.value().to_string(),
                    ])
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
        self.finish();
    }
}
