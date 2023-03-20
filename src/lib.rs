use include_dir::Dir;

pub fn recursive_replace(dir: Dir, pattern_val_pairs: &[(&str, &String)]) {
    for entry in dir.entries() {
        match entry {
            include_dir::DirEntry::File(file) => {
                let file_raw = file
                    .contents_utf8()
                    .expect("failure at existance of `contents_utf8`")
                    .to_string();

                let file_h = pattern_val_pairs
                    .iter()
                    .fold(file_raw, |acc, pair| acc.replace(pair.0, pair.1));

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
