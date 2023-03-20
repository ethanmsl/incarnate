pub fn recursive_replace(dir: include_dir::Dir, pattern_val_pairs: &[(&str, &String)]) {
    for entry in dir.entries() {
        match entry {
            include_dir::DirEntry::File(file) => {
                let file_h = match replace_file_contents(file, pattern_val_pairs) {
                    Some(file_h) => file_h,
                    None => {
                        continue;
                    }
                };

                println!("Writing file to {:?}", file.path());
                std::fs::create_dir_all(file.path().parent().expect("no parent"))
                    .expect("unable to create dir");
                std::fs::write(file.path(), file_h).expect("unable to write file");
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
