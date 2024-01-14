//! Skeleton Repo Incarnater
//! API for Skeleton Repo Use
//!
//! # Build Note:
//!     `.DS_Store` files have caused errors before
//!     presently I'm just deleting them
//!     ```find . -name ".DS_Store" -delete```

use std::path::Path;

use anyhow::Context;
use clap::Parser;
use incarnate::{shell_actions, template_populator};
use include_dir::{include_dir, Dir};
use tracing::{event, Level};

static ASSETS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets");

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct InputStruct {
        project_name: String,
        cli_app_name: Option<String>,

        #[clap(short='a', long="author", default_value_t=String::from("author_name_not_supplied"))]
        author_name:       String,
        #[clap(short='e', long="email", default_value_t=String::from("email_not_supplied"))]
        no_reply_email:    String,
        #[clap(short = 'c', long = "coverage_minimum", default_value_t = 50)]
        test_coverage_min: u8,
}

#[tracing::instrument]
fn main() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();

        let user_input = InputStruct::parse();
        event!(Level::DEBUG, ?user_input, "User input received:");

        let replacement_pairs = [("${{ carnate.project_name }}", &user_input.project_name),
                                 ("${{ carnate.author_name }}", &user_input.author_name),
                                 ("${{ carnate.no_reply_email }}", &user_input.no_reply_email),
                                 ("${{ carnate.cli_app_name }}",
                                  &user_input.cli_app_name
                                             .unwrap_or(user_input.project_name
                                                                  .clone())),
                                 ("${{ carnate.test_coverage_min }}",
                                  &user_input.test_coverage_min
                                             .to_string())];
        event!(Level::DEBUG,replacement_pairs = ?replacement_pairs, "Template Field:Value Pairs:");

        println!("-------------");
        println!("Writing files to {:?}", user_input.project_name);
        // copy PROJECT_DIR to a current directory
        // NOTE: this can probably be removed on refactor with proper referencing of `ASSETS_DIR`
        let path = format!("parent/{}/", user_input.project_name);
        event!(Level::DEBUG, path = ?path,"Path passed in for new directory location: ");
        // TODO: add checks for ASSETS_DIR.entries() values as valid & directory creation
        // NOTE: ASSETS_DIR is a git-submodule, needs local init & update in fresh repo
        let new_dir_copy = Dir::new(&path, ASSETS_DIR.entries());
        event!(Level::TRACE, new_dir_copy = ?new_dir_copy, "Newly created directory:");

        template_populator::recursive_replace(new_dir_copy, &replacement_pairs)?;
        let proj_relative_path = Path::new(&user_input.project_name);
        event!(Level::DEBUG,proj_relative_path = ?proj_relative_path,"Passing project path to shell actions:");
        shell_actions::git_setup(proj_relative_path).context("Failed to perform git repo setup")?;

        event!(Level::INFO, "Incarnate script complete.");
        Ok(())
}
