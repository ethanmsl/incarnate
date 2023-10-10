//! Collection of actions that use the shell directly.
//!
//! # Examples
//! - the `pre-commit` hook is created as a non-executable file
//! - running `git init` helps ensure users normal git settings are respected

use std::env;
use std::io;
use std::path::Path;
use std::process::Command;
use tracing::debug;

/// runs a series of shell commands to initialize a git repo
pub fn git_setup(path: &Path) -> io::Result<()> {
        git_init(path)?;
        move_pre_commit_hook(path)?;
        git_add_all(path)?;
        git_initial_commit(path)?;
        Ok(())
}

/// runs `git init` in a given directory
fn git_init(path: &Path) -> io::Result<()> {
        let pathstring = path.to_str().expect("Failed to convert path to string");
        let cwd = get_current_working_dir();
        let abs_path_proj = cwd + "/" + pathstring;
        debug!( abs_path_proj = ?abs_path_proj, "git_init");

        let mut git_cmd = Command::new("git");
        git_cmd.arg("init");
        git_cmd.current_dir(abs_path_proj)
                .output()
                .expect("Failed to run git init");
        Ok(())
}

/// runs `git add .` in a given directory
fn git_add_all(path: &Path) -> io::Result<()> {
        let pathstring = path.to_str().expect("Failed to convert path to string");
        let cwd = get_current_working_dir();
        let abs_path_proj = cwd + "/" + pathstring;
        debug!(abs_path_proj = ?abs_path_proj,"git_add_all");

        let mut git_cmd = Command::new("git");
        git_cmd.arg("add").arg(".");
        git_cmd.current_dir(abs_path_proj)
                .output()
                .expect("Failed to run git add .");
        Ok(())
}

/// runs a commit with "initial commit" in a given directory
/// ignores git hooks
fn git_initial_commit(path: &Path) -> io::Result<()> {
        let pathstring = path.to_str().expect("Failed to convert path to string");
        let cwd = get_current_working_dir();
        let abs_path_proj = cwd + "/" + pathstring;
        debug!(abs_path_proj = ?abs_path_proj,"git_initial_commit");

        let mut git_cmd = Command::new("git");
        git_cmd.arg("commit")
                .arg("--message")
                .arg("Initial commit")
                .arg("--no-verify");
        git_cmd.current_dir(abs_path_proj)
                .output()
                .expect("Failed to run git commit");
        Ok(())
}

/// makes executable and moves a pre-commit hook
/// by original intention from the root of a created project directory to
/// the `.git/hooks` dir of a recently created git instance
fn move_pre_commit_hook(path: &Path) -> io::Result<()> {
        let pathstring = path.to_str().expect("Failed to convert path to string");
        let cwd = get_current_working_dir();
        let abs_path_proj = cwd + "/" + pathstring;

        let mut chmod_cmd = Command::new("chmod");
        chmod_cmd.arg("+x").arg("pre-commit");
        chmod_cmd
                .current_dir(abs_path_proj.clone())
                .output()
                .expect("Failed to run chmod +x pre-commit");

        let mut mv_cmd = Command::new("mv");
        mv_cmd.arg("pre-commit").arg(".git/hooks/pre-commit");
        mv_cmd.current_dir(abs_path_proj)
                .output()
                .expect("Failed to run mv pre-commit .git/hooks/pre-commit");
        Ok(())
}

/// gets workind directory and returns as a `String`
fn get_current_working_dir() -> String {
        let res = env::current_dir();
        match res {
                Ok(path) => path.into_os_string().into_string().unwrap(),
                Err(_) => "FAILED".to_string(),
        }
}
