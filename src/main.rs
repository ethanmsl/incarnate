//! Skeleton Repo Incarnater
//! API for Skeleton Repo Use
//!
//! # Build Note:
//!     `.DS_Store` files have caused errors before
//!     presently I'm just deleting them
//!     ```find . -name ".DS_Store" -delete```

use clap::Parser;
use clap_interactive::*;
use incarnate::{shell_actions, template_populator};
use include_dir::{include_dir, Dir};
use std::path::Path;
use struct_field_names_as_array::FieldNamesAsArray;
use tracing::{debug, info, trace};

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
    // install global collector configured based on `RUST_LOG` env var.
    //     `RUST_LOG=info cargo run`
    //     `RUST_LOG ∊ {trace,debug,info,warn,error}`
    //  NOTE: `tracing-log` feature enabled, should be able to consume `log` events
    tracing_subscriber::fmt::init();

    let user_input = SomeStruct::interactive_parse().expect("unable to parse user input");
    info!(user_input = ?user_input, "User input received:");

    let _replacement_tokens = SomeStruct::FIELD_NAMES_AS_ARRAY
        .iter()
        .map(|&s| format!("${{ {} }}", s))
        .collect::<Vec<String>>();
    trace!(
        _replacement_tokens = ?_replacement_tokens,
        "Field names rendered as array (NOTE: unused)"
    );

    // no way to guarantee macro derived struct name ordering and field iteration match
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
    info!(replacement_pairs = ?replacement_pairs, "Template Field:Value Pairs:");

    println!("-------------");
    println!("Writing files to {:?}", user_input.project_name);
    // copy PROJECT_DIR to a current directory
    // NOTE: this can probably be removed on refactor with proper referencing of `ASSETS_DIR`
    let path = format!("parent/{}/", user_input.project_name);
    debug!(path = ?path,"Path passed in for new directory location: ");
    let new_dir_copy = Dir::new(&path, ASSETS_DIR.entries());
    trace!(new_dir_copy = ?new_dir_copy, "Newly created directory:");

    template_populator::recursive_replace(new_dir_copy, &replacement_pairs);
    let proj_relative_path = Path::new(&user_input.project_name);
    debug!(proj_relative_path = ?proj_relative_path,"Passing project path to shell actions:");
    shell_actions::git_setup(proj_relative_path).expect("Failed to perform git repo setup");

    info!("Incarnate script complete.");
}
