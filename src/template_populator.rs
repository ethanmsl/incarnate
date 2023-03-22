//!
//! Core element that recursively navigates templated assets and replaces variables
//! of the form `${{ carnate.____ }}`

use std::path::Path;

/// works through a nested Enum or corresponding Dir & File value
/// replacing patterns present in files or in path names
pub fn recursive_replace(dir: include_dir::Dir, pattern_val_pairs: &[(&str, &String)]) {
    for entry in dir.entries() {
        match entry {
            include_dir::DirEntry::File(file) => {
                let hydrated_string = replace_file_contents(file, pattern_val_pairs)
                    .expect("unable to find utf8 file contents");

                let pathstring = file
                    .path()
                    .to_str()
                    .expect("unable to convert path to str")
                    .replace(pattern_val_pairs[0].0, pattern_val_pairs[0].1)
                    .replace("${{ python-skeleton }}", pattern_val_pairs[0].1);
                // got into some opaque work regarding lifetime counting
                // so leaveing this here for right now

                let path = Path::new(&pathstring);
                write_file(path, hydrated_string);
            }
            include_dir::DirEntry::Dir(dir) => {
                recursive_replace(dir.clone(), pattern_val_pairs);
            }
        }
    }
}

/// checks file contents for any of an array of str patterns
/// and replace them with a pair value if present
fn replace_file_contents(
    file: &include_dir::File,
    pattern_val_pairs: &[(&str, &String)],
) -> Option<String> {
    match file.contents_utf8() {
        Some(file_raw) => {
            let hydrated_file = pattern_val_pairs
                .iter()
                .fold(file_raw.to_string(), |acc, pair| {
                    acc.replace(pair.0, pair.1)
                });
            Some(hydrated_file)
        }
        None => None,
    }
}

/// writes files, creating directories as needed
fn write_file(filepath: &Path, hydrated_string: String) {
    std::fs::create_dir_all(filepath.parent().expect("no parent")).expect("unable to create dir");
    std::fs::write(filepath, hydrated_string).expect("unable to write file");
}
