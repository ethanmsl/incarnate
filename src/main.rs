//! Skeleton Repo Incarnater
//! API for Skeleton Repo Use
//!
//! # Build Note:
//!     `.DS_Store` files have caused errors before
//!     presently I'm just deleting them
//!     ```find . -name ".DS_Store" -delete```

#![allow(clippy::uninlined_format_args)]
use clap::Parser;
use clap_interactive::*;
use include_dir::{include_dir, Dir};
use struct_field_names_as_array::FieldNamesAsArray;

static ASSETS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets");

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[derive(FieldNamesAsArray)]
struct SomeStruct {
    project_name: String,
    author_name: String,
    no_reply_email: String,
    cli_app_name: String,
    test_coverage_min: u8,
}

fn main() {
    let user_input = SomeStruct::interactive_parse().expect("unable to parse user input");
    let _replacement_tokens = SomeStruct::FIELD_NAMES_AS_ARRAY
        .iter()
        .map(|&s| format!("${{ {} }}", s))
        .collect::<Vec<String>>();

    let cli_app_name_hydrated = format!(
        "\n\n# declares name of CLI app and where the insertion point is\n[tool.poetry.scripts]\n{} = \"pkg_name.commands:app\"",
        &user_input.cli_app_name);

    // no way to guaranteed that macro derived struct name ordering and
    // struct field iteration match
    // forces manual entry
    let replacement_pairs = [
        ("${{ carnate.project_name }}", &user_input.project_name),
        ("${{ carnate.author_name }}", &user_input.author_name),
        ("${{ carnate.no_reply_email }}", &user_input.no_reply_email),
        ("${{ carnate.cli_app_name }}", &cli_app_name_hydrated),
        (
            "${{ carnate.test_coverage_min }}",
            &user_input.test_coverage_min.to_string(),
        ),
    ];

    println!("-------------");
    println!("Writing files to {:?}", user_input.project_name);
    // copy PROJECT_DIR to a current directory
    let path = format!("parent/{}/", user_input.project_name);

    let newb = Dir::new(&path, ASSETS_DIR.entries());
    println!("newb: {:?}", newb);

    recursive_replace(newb, &user_input.project_name, &replacement_pairs);
}

fn recursive_replace(dir: Dir, name: &str, pattern_val_pairs: &[(&str, &String)]) {
    for entry in dir.entries() {
        match entry {
            include_dir::DirEntry::File(file) => {
                let file_raw = file
                    .contents_utf8()
                    .expect("failure at existance of `contents_utf8`");
                let file_hydrated = file_raw.replace("${{ unfindable.name }}", name);

                let file_h = pattern_val_pairs
                    .iter()
                    .fold(file_hydrated, |acc, pair| acc.replace(pair.0, pair.1));

                println!("Writing file to {:?}", file.path());
                std::fs::create_dir_all(file.path().parent().expect("no parent"))
                    .expect("unable to create dir");
                std::fs::write(file.path(), file_h).expect("unable to write file");
            }
            include_dir::DirEntry::Dir(dir) => {
                recursive_replace(dir.clone(), name, pattern_val_pairs);
            }
        }
    }
}
