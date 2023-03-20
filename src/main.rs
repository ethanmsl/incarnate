//! demo from front page of [clap-interactive](https://lib.rs/crates/clap-interactive)
#![allow(clippy::uninlined_format_args)]
use clap::Parser;
use clap_interactive::*;
use include_dir::{include_dir, Dir};

static ASSETS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets");

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct SomeStruct {
    #[command(subcommand)]
    subcommand: SubCommand,
    arg: String,
}

#[derive(Parser, Debug)]
#[clap(rename_all = "snake_case", infer_subcommands = true)]
enum SubCommand {
    Commit { message: String },
    Clone { address: String },
    Boogita { num: u32 },
}

fn main() {
    let git = SomeStruct::interactive_parse().unwrap();
    println!("{:?}", git);

    // // of course, you can retrieve a file by its full path
    // let asst_tree = PROJECT_DIR.get_file("assets/").unwrap();

    // copy PROJECT_DIR to a current directory
    ASSETS_DIR.extract("created-assets/").unwrap();
}
