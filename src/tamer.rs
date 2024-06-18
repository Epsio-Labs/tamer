use std::path::PathBuf;
use crate::utils::cargo_target_directory;
use crate::group::Group;

pub struct Tamer {
    groups: Vec<Group>,
    output_dir: Option<PathBuf>,
}

impl Tamer {
    pub fn new(output_dir: Option<PathBuf>) -> Tamer {
        Tamer {
            groups: Vec::new(),
            output_dir
        }
    }
    pub fn benchmark_group<S: ToString>(&mut self, name: S) -> &mut Group {
        let g = Group::new(name.to_string(), self.output_dir.clone());
        self.groups.push(g);
        self.groups.last_mut().unwrap()
    }
}

impl Default for Tamer {
    fn default() -> Self {
        Tamer { groups: Vec::new(), output_dir: cargo_target_directory() }
    }
}
