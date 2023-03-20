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

    // no way to guaranteed that macro derived struct name ordering and
    // struct field iteration match
    // forces manual entry
    let replacement_pairs = [
        ("${{ carnate.project_name }}", &user_input.project_name),
        ("${{ carnate.author_name }}", &user_input.author_name),
        ("${{ carnate.no_reply_email }}", &user_input.no_reply_email),
        ("${{ carnate.cli_app_name }}", &user_input.cli_app_name),
        (
            "${{ carnate.test_coverage_min }}",
            &user_input.test_coverage_min.to_string(),
        ),
    ];

    println!("-------------");
    println!("Writing files to {:?}", user_input.project_name);
    // copy PROJECT_DIR to a current directory
    // NOTE: this can probably be removed on refactor with proper referencing of `ASSETS_DIR`
    let path = format!("parent/{}/", user_input.project_name);
    let new_dir_copy = Dir::new(&path, ASSETS_DIR.entries());

    incarnate::recursive_replace(new_dir_copy, &replacement_pairs);
}
