//! # Dev Examples - example `walkdir` code
//! Note: These examples are here for dev use and functionality testing.
//!
//! Currently looking at `walkdir` to replace recursive walker that the original code used.

use walkdir::WalkDir;

fn main() -> anyhow::Result<()> {
        for entry in WalkDir::new("assets") {
                let entry = entry?;
                println!("{}",
                         entry.path()
                              .display());
        }
        Ok(())
}
