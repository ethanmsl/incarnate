//!
//! Core element that recursively navigates templated assets and replaces variables
//! of the form `${{ carnate.____ }}`

use std::path::Path;

use color_eyre::eyre::{ContextCompat, Result, WrapErr};
use include_dir::DirEntry::{Dir, File};
use tracing::{event, Level};

/// works through a nested Enum or corresponding Dir & File value
/// replacing patterns present in files or in path names
#[tracing::instrument(skip(dir, pattern_val_pairs))]
pub fn recursive_replace(dir: include_dir::Dir,
                         pattern_val_pairs: &[(&str, &String)])
                         -> Result<()> {
        for entry in dir.entries() {
                match entry {
                        File(submod_git)
                                if submod_git.path()
                                             .to_str()
                                             .context("")?
                                             .ends_with(".git") =>
                        {
                                event!(Level::DEBUG, ?submod_git, "skipping file, as it is an artifact of the git submodule embedding of the assets");
                                continue
                        },
                        File(file) => {
                                let pathstring =
                                        file.path()
                                            .to_str()
                                            .context("unable to convert path to str")?
                                            .replace(pattern_val_pairs[0].0, pattern_val_pairs[0].1)
                                            .replace("${{ carnate.python-skeleton }}",
                                                     pattern_val_pairs[0].1);
                                event!(Level::TRACE, ?pathstring, "pathstring");
                                let path = Path::new(&pathstring);

                                // bit hackish, I should test for UTF8-ness first and then replace
                                let maybe_hydrated = replace_file_contents(file, pattern_val_pairs);
                                match maybe_hydrated {
                                        Some(hydrated_string) => {
                                                event!(Level::TRACE,
                                                       ?hydrated_string,
                                                       "hydrated_string");

                                                write_string_file(path, hydrated_string)?;
                                                event!(Level::TRACE,
                                                       ?path,
                                                       "write path, string file");
                                        },
                                        None => {
                                                event!(Level::WARN, ?path, "file was not hydratable, likely not UTF8 valid encoding");
                                                write_bin_file(path, file)?;
                                                event!(Level::TRACE, ?path, "write path, bin file");
                                        },
                                }
                        },
                        Dir(dir) => {
                                recursive_replace(dir.clone(), pattern_val_pairs)?;
                        },
                }
        }
        Ok(())
}

/// checks file contents for any of an array of str patterns
/// and replace them with a pair value if present
#[tracing::instrument]
fn replace_file_contents(file: &include_dir::File,
                         pattern_val_pairs: &[(&str, &String)])
                         -> Option<String> {
        match file.contents_utf8() {
                Some(file_raw) => {
                        let hydrated_file =
                                pattern_val_pairs.iter()
                                                 .fold(file_raw.to_string(), |acc, pair| {
                                                         acc.replace(pair.0, pair.1)
                                                 });
                        Some(hydrated_file)
                },
                None => None,
        }
}

/// writes files from strings, creating directories as needed
#[tracing::instrument]
fn write_string_file(filepath: &Path, hydrated_string: String) -> Result<()> {
        std::fs::create_dir_all(filepath.parent()
                                        .context("no parent")?).context("unable to create dir")?;
        std::fs::write(filepath, hydrated_string).context("unable to write file")?;
        event!(Level::TRACE,
               ?filepath,
               "wrote to filepath from string representation:");
        Ok(())
}

/// Writes binary files, creating directories as needed.
#[tracing::instrument]
fn write_bin_file(filepath: &Path, bin_file: &include_dir::File) -> Result<()> {
        // Ensure the directory exists where the file will be written
        std::fs::create_dir_all(filepath.parent().context("No parent directory for the file")?)
        .context("Unable to create directory for the file")?;

        // Write the binary contents directly to the file
        std::fs::write(filepath, bin_file.contents()).context("Unable to write binary file")?;

        event!(Level::TRACE, ?filepath, "Wrote to filepath as binary file");
        Ok(())
}
