/// Collection of actions that use the shell directly.
///
/// # Examples
/// - the `pre-commit` hook is created as a non-executable file
/// - running `git init` helps ensure users normal git settings are respected
use std::env;
use std::io;
use std::path::Path;
use std::process::Command;

pub fn git_setup(path: &Path) -> io::Result<()> {
    git_init(path)?;
    git_add_all(path)?;
    git_initial_commit(path)?;
    move_pre_commit_hook(path)?;
    Ok(())
}

fn git_init(path: &Path) -> io::Result<()> {
    let pathstring = path.to_str().expect("Failed to convert path to string");
    let cwd = get_current_working_dir();
    let abs_path_proj = cwd + "/" + pathstring;

    let mut git_cmd = Command::new("git");
    git_cmd.arg("init");
    git_cmd.current_dir(abs_path_proj).output().unwrap();
    Ok(())
}

fn git_add_all(path: &Path) -> io::Result<()> {
    let pathstring = path.to_str().expect("Failed to convert path to string");
    let cwd = get_current_working_dir();
    let abs_path_proj = cwd + "/" + pathstring;

    let mut git_cmd = Command::new("git");
    git_cmd.arg("add").arg(".");
    git_cmd.current_dir(abs_path_proj).output().unwrap();
    Ok(())
}

fn git_initial_commit(path: &Path) -> io::Result<()> {
    let pathstring = path.to_str().expect("Failed to convert path to string");
    let cwd = get_current_working_dir();
    let abs_path_proj = cwd + "/" + pathstring;

    let mut git_cmd = Command::new("git");
    git_cmd.arg("commit").arg("--message").arg("Initial commit");
    git_cmd.current_dir(abs_path_proj).output().unwrap();
    Ok(())
}

fn move_pre_commit_hook(path: &Path) -> io::Result<()> {
    let pathstring = path.to_str().expect("Failed to convert path to string");
    let cwd = get_current_working_dir();
    let abs_path_proj = cwd + "/" + pathstring;

    let mut chmod_cmd = Command::new("chmod");
    chmod_cmd.arg("+x").arg("pre-commit");
    chmod_cmd
        .current_dir(abs_path_proj.clone())
        .output()
        .unwrap();

    let mut mv_cmd = Command::new("mv");
    mv_cmd.arg("pre-commit").arg(".git/hooks/pre-commit");
    mv_cmd.current_dir(abs_path_proj).output().unwrap();
    Ok(())
}

fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string(),
    }
}
