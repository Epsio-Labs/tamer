use std::env;
use std::path::PathBuf;
use std::process::Command;
use serde_derive::Deserialize;

pub fn cargo_target_directory() -> Option<PathBuf> {
    #[derive(Deserialize)]
    struct Metadata {
        target_directory: PathBuf,
    }

    env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .or_else(|| {
            let output = Command::new(env::var_os("CARGO")?)
                .args(["metadata", "--format-version", "1"])
                .output()
                .ok()?;
            let metadata: Metadata = serde_json::from_slice(&output.stdout).ok()?;
            Some(metadata.target_directory)
        })
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