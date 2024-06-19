use std::path::PathBuf;
use regex::Regex;
use crate::utils::cargo_target_directory;
use crate::group::Group;

pub struct Tamer {
    groups: Vec<Group>,
    output_dir: Option<PathBuf>,
    filter: BenchmarkFilter,
}


/// Benchmark filtering support.
#[derive(Clone, Debug)]
pub enum BenchmarkFilter {
    /// Run all benchmarks.
    AcceptAll,
    /// Run benchmarks matching this regex.
    Regex(Regex),
    /// Run the benchmark matching this string exactly.
    Exact(String),
    /// Do not run any benchmarks.
    RejectAll,
}

impl BenchmarkFilter {
    pub fn filter_matches(&self, id: &str) -> bool {
        match self {
            BenchmarkFilter::AcceptAll => true,
            BenchmarkFilter::Regex(regex) => regex.is_match(id),
            BenchmarkFilter::Exact(exact) => id == exact,
            BenchmarkFilter::RejectAll => false,
        }
    }
}

impl Tamer {
    pub fn new(mut output_dir: Option<PathBuf>, filter: BenchmarkFilter) -> Tamer {
        output_dir = output_dir.map(|a| a.join("tamer"));
        Tamer {
            groups: Vec::new(),
            output_dir,
            filter,
        }
    }
    pub fn benchmark_group<S: ToString>(&mut self, name: S) -> &mut Group {
        let name = name.to_string();
        if self.groups.iter().any(|g| g.name == name) {
            panic!("Benchmark group with name '{}' already exists", &name);
        }
        let g = Group::new(name, self.output_dir.clone());
        self.groups.push(g);
        self.groups.last_mut().unwrap()
    }

    pub fn benchmark(&mut self) {
        for group in self.groups.iter_mut() {
            group.benchmark(&self.filter);
            group.report();
        }
    }

    /// Only run benchmarks specified by the given filter.
    pub fn with_benchmark_filter(mut self, filter: BenchmarkFilter) -> Tamer {
        self.filter = filter;

        self
    }

    pub fn configure_from_args(mut self) -> Tamer {
        use clap::{Arg, Command};
        let matches = Command::new("Tamer Benchmark")
            .arg(Arg::new("bench")
                .hide(true)
                .long("bench")
                .num_args(0))
            .arg(Arg::new("FILTER")
                .help("Skip benchmarks whose names do not contain FILTER.")
                .index(1))
            .arg(Arg::new("exact")
                .long("exact")
                .num_args(0)
                .help("Run benchmarks that exactly match the provided filter"))
            .arg(Arg::new("ignored")
                .long("ignored")
                .num_args(0)
                .help("List or run ignored benchmarks (currently means skip all benchmarks)")).get_matches();

        let filter = if matches.get_flag("ignored") {
            // --ignored overwrites any name-based filters passed in.
            BenchmarkFilter::RejectAll
        } else if let Some(filter) = matches.get_one::<String>("FILTER") {
            if matches.get_flag("exact") {
                BenchmarkFilter::Exact(filter.to_owned())
            } else {
                let regex = Regex::new(filter).unwrap_or_else(|err| {
                    panic!(
                        "Unable to parse '{}' as a regular expression: {}",
                        filter, err
                    )
                });
                BenchmarkFilter::Regex(regex)
            }
        } else {
            BenchmarkFilter::AcceptAll
        };
        self = self.with_benchmark_filter(filter);
        self
    }
}

impl Default for Tamer {
    fn default() -> Self {
        Tamer::new(cargo_target_directory(), BenchmarkFilter::AcceptAll)
    }
}
