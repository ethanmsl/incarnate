use std::path::Path;

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
                    .replace(pattern_val_pairs[0].0, pattern_val_pairs[0].1);
                let path = Path::new(&pathstring);
                write_file(path, hydrated_string);
            }
            include_dir::DirEntry::Dir(dir) => {
                recursive_replace(dir.clone(), pattern_val_pairs);
            }
        }
    }
}

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

fn write_file(filepath: &Path, hydrated_string: String) {
    println!("Writing file to {:?}", filepath);
    std::fs::create_dir_all(filepath.parent().expect("no parent")).expect("unable to create dir");
    std::fs::write(filepath, hydrated_string).expect("unable to write file");
}
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
