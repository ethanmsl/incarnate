//! Skeleton Repo Incarnater
//! API for Skeleton Repo Use

//
//
// # # declares name of CLI app and where the insertion point is
// # [tool.poetry.scripts]
// # cli-app-name = "pkg_name.commands:app"

#![allow(clippy::uninlined_format_args)]
use clap::Parser;
use clap_interactive::*;
use include_dir::{include_dir, Dir};

static ASSETS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets");

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct SomeStruct {
    project_name: String,
    author_name: String,
    no_reply_email: String,
    cli_app_name: Option<String>,
    test_coverage_min: u8,
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
    println!("Writing files to {:?}", user_input.project_name);
    // copy PROJECT_DIR to a current directory
    let path = format!("parent/{}/", user_input.project_name);

    // ASSETS_DIR.extract(path).unwrap();
    let newb = Dir::new(&path, ASSETS_DIR.entries());
    println!("newb: {:?}", newb);
    newb.extract("blaaaap").unwrap();

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
