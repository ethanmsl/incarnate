/// Collection of actions that use the shell directly.
///
/// # Examples
/// - the `pre-commit` hook is created as a non-executable file
/// - running `git init` helps ensure users normal git settings are respected
use std::env;
use std::io;
use std::path::Path;
use std::process::Command;

// fn git_init() -> io::Result<()> {
//     // bind git command
//     // init in project directory just created
//     let mut git_cmd = Command::new("git");
//     git_cmd.arg("init");
//
//     git_cmd.output()?;
//     Ok(())
// }
//
// fn git_add_all() -> io::Result<()> {
//     // bind git command
//     // add all files in project directory just created
//     let mut git_cmd = Command::new("git");
//     git_cmd.arg("add").arg(".");
//
//     git_cmd.output()?;
//     Ok(())
// }
//
// fn git_initial_commit() -> io::Result<()> {
//     // bind git command
//     // add all files in project directory just created
//     let mut git_cmd = Command::new("git");
//     git_cmd.arg("commit").arg("--message");
//
//     git_cmd.output()?;
//     Ok(())
// }

pub fn git_init(path: &Path) -> io::Result<()> {
    let pathstring = path.to_str().expect("Failed to convert path to string");
    let cwd = get_current_working_dir();
    let abs_path_proj = cwd + "/" + pathstring;

    let mut git_cmd = Command::new("git");
    git_cmd.arg("init");
    git_cmd.current_dir(abs_path_proj).output().unwrap();
    Ok(())
}

fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string(),
    }
}
