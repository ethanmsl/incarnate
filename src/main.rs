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
    subcommand_field: SubCommand,
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
    let user_input = SomeStruct::interactive_parse().unwrap();
    println!("{:?}", user_input);

    /////////////////////////////////

    let assets = ASSETS_DIR.clone();

    let entries = ASSETS_DIR.entries();

    println!("-------------");
    for entry in entries {
        println!("{:?}\n\n", entry.children());
        match entry {
            include_dir::DirEntry::File(file) => {
                println!("FILE: {:?}\n\n", file);
            }
            include_dir::DirEntry::Dir(dir) => {
                println!("DIR: {:?}\n\n", dir);
            }
        }
    }

    println!("-------------");
    recursive_print(assets);

    println!("-------------");
    println!("-------------");
    println!("-------------");
    println!("Writing files to {:?}", user_input.arg);
    // copy PROJECT_DIR to a current directory
    let path = format!("{}/", user_input.arg);
    ASSETS_DIR.extract(path).unwrap();
}

fn recursive_print(dir: Dir) {
    for entry in dir.entries() {
        match entry {
            include_dir::DirEntry::File(file) => {
                println!("FILE: {:?}\n\n", file);
            }
            include_dir::DirEntry::Dir(dir) => {
                println!("DIR: {:?}\n\n", dir);
                recursive_print(dir.clone());
            }
        }
    }
}
