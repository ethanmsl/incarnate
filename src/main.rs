//! Skeleton Repo Incarnater
//! API for Skeleton Repo Use
//!
//! # Build Note:
//!     `.DS_Store` files have caused errors before
//!     presently I'm just deleting them
//!     ```find . -name ".DS_Store" -delete```

//
//
// # # declares name of CLI app and where the insertion point is
// # [tool.poetry.scripts]
// # cli-app-name = "pkg_name.commands:app"

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
    let user_input = SomeStruct::interactive_parse().unwrap();
    let replacement_tokens = SomeStruct::FIELD_NAMES_AS_ARRAY
        .iter()
        .map(|&s| format!("${{ {} }}", s))
        .collect::<Vec<String>>();

    println!("{:?}", replacement_tokens);

    println!("-------------");
    println!("Writing files to {:?}", user_input.project_name);
    // copy PROJECT_DIR to a current directory
    let path = format!("parent/{}/", user_input.project_name);

    let newb = Dir::new(&path, ASSETS_DIR.entries());
    println!("newb: {:?}", newb);

    recursive_replace(newb, &user_input.project_name);
}

fn recursive_replace(dir: Dir, name: &str) {
    for entry in dir.entries() {
        match entry {
            include_dir::DirEntry::File(file) => {
                let input = file.contents_utf8().unwrap();
                let output = input.replace("${{ name }}", name);
                println!("Writing file to {:?}", file.path());
                std::fs::create_dir_all(file.path().parent().unwrap()).unwrap();
                std::fs::write(file.path(), output).unwrap();
            }
            include_dir::DirEntry::Dir(dir) => {
                recursive_replace(dir.clone(), name);
            }
        }
    }
}
