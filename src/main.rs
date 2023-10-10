//! Skeleton Repo Incarnater
//! API for Skeleton Repo Use
//!
//! # Build Note:
//!     `.DS_Store` files have caused errors before
//!     presently I'm just deleting them
//!     ```find . -name ".DS_Store" -delete```

use clap::Parser;
use incarnate::{shell_actions, template_populator};
use include_dir::{include_dir, Dir};
use std::path::Path;
use tracing::{debug, info, trace};

// TODO: add checks for extant directory
//       AND check/warning re: git-submodule initialization
static ASSETS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets");

#[derive(Parser, Debug)]
#[clap(author, version, about)] // from: `authors`, `version`, `about`
struct InputStruct {
        project_name: String,
        cli_app_name: Option<String>,

        #[clap(short='a', long="author", default_value_t = String::from("author_name_not_supplied"))]
        author_name: String,
        #[clap(short='e', long="email", default_value_t = String::from("email_not_supplied"))]
        no_reply_email: String,
        #[clap(short = 'c', long = "coverage_minimum", default_value_t = 80)]
        test_coverage_min: u8,
}

fn main() {
        // install global collector configured based on `RUST_LOG` env var.
        //     `RUST_LOG=info cargo run`
        //     `RUST_LOG ∊ {trace,debug,info,warn,error}`
        //  NOTE: `tracing-log` feature enabled, should be able to consume `log` events
        tracing_subscriber::fmt::init();

        let user_input = InputStruct::parse();
        info!(user_input = ?user_input, "User input received:");

        let replacement_pairs = [
                ("${{ carnate.project_name }}", &user_input.project_name),
                ("${{ carnate.author_name }}", &user_input.author_name),
                ("${{ carnate.no_reply_email }}", &user_input.no_reply_email),
                (
                        "${{ carnate.cli_app_name }}",
                        &user_input
                                .cli_app_name
                                .unwrap_or(user_input.project_name.clone()),
                ),
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
        // TODO: add checks for ASSETS_DIR.entries() values as valid & directory creation
        // NOTE: ASSETS_DIR is a git-submodule, needs local init & update in fresh repo
        let new_dir_copy = Dir::new(&path, ASSETS_DIR.entries());
        trace!(new_dir_copy = ?new_dir_copy, "Newly created directory:");

        template_populator::recursive_replace(new_dir_copy, &replacement_pairs);
        let proj_relative_path = Path::new(&user_input.project_name);
        debug!(proj_relative_path = ?proj_relative_path,"Passing project path to shell actions:");
        shell_actions::git_setup(proj_relative_path).expect("Failed to perform git repo setup");

        info!("Incarnate script complete.");
}
