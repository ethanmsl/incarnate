//! Collection of actions that use the shell directly.
//!
//! # Examples
//! - the `pre-commit` hook is created as a non-executable file
//! - running `git init` helps ensure users normal git settings are respected

use anyhow::Context;
use std::env;
use std::io;
use std::path::Path;
use std::process::Command;
use tracing::{event, Level};

/// runs a series of shell commands to initialize a git repo
#[tracing::instrument]
pub fn git_setup(path: &Path) -> anyhow::Result<()> {
        git_init(path)?;
        move_pre_commit_hook(path)?;
        git_add_all(path)?;
        git_initial_commit(path)?;
        Ok(())
}

/// runs `git init` in a given directory
#[tracing::instrument]
fn git_init(path: &Path) -> anyhow::Result<()> {
        let pathstring = path.to_str().context("Failed to convert path to string")?;
        let cwd = get_current_working_dir()?;
        let abs_path_proj = cwd + "/" + pathstring;

        event!(Level::DEBUG, ?abs_path_proj, "git_init path is:");

        let mut git_cmd = Command::new("git");
        git_cmd.arg("init");
        let git_init_out = git_cmd
                .current_dir(abs_path_proj)
                .output()
                .context("Failed to run git init")?;
        // BUG: status 128 - owner of local repo does not match runner of command
        event!(
                Level::DEBUG,
                %git_init_out.status,
                "git_init, status:"
        );

        Ok(())
}

/// runs `git add .` in a given directory
#[tracing::instrument]
fn git_add_all(path: &Path) -> anyhow::Result<()> {
        let pathstring = path.to_str().context("Failed to convert path to string")?;
        let cwd = get_current_working_dir()?;
        let abs_path_proj = cwd + "/" + pathstring;
        event!(Level::DEBUG, ?abs_path_proj, "git_add_all");

        let mut git_cmd = Command::new("git");
        git_cmd.arg("add").arg(".");
        git_cmd.current_dir(abs_path_proj)
                .output()
                .context("Failed to run git add .")?;
        Ok(())
}

/// runs a commit with "initial commit" in a given directory
/// ignores git hooks
#[tracing::instrument]
fn git_initial_commit(path: &Path) -> anyhow::Result<()> {
        let pathstring = path.to_str().context("Failed to convert path to string")?;
        let cwd = get_current_working_dir()?;
        let abs_path_proj = cwd + "/" + pathstring;
        event!(Level::DEBUG, ?abs_path_proj, "git_initial_commit");

        let mut git_cmd = Command::new("git");
        git_cmd.arg("commit")
                .arg("--message")
                .arg("Initial commit")
                .arg("--no-verify");
        git_cmd.current_dir(abs_path_proj)
                .output()
                .context("Failed to run git commit")?;
        Ok(())
}

/// makes executable and moves a pre-commit hook
/// by original intention from the root of a created project directory to
/// the `.git/hooks` dir of a recently created git instance
#[tracing::instrument]
fn move_pre_commit_hook(path: &Path) -> anyhow::Result<()> {
        let pathstring = path.to_str().context("Failed to convert path to string")?;
        let cwd = get_current_working_dir()?;
        let abs_path_proj = cwd + "/" + pathstring;

        let mut chmod_cmd = Command::new("chmod");
        chmod_cmd.arg("+x").arg("pre-commit");
        let chmod_out = chmod_cmd
                .current_dir(abs_path_proj.clone())
                .output()
                .context("Failed to run chmod +x pre-commit")?;
        event!(
                Level::DEBUG,
                chmod_status = ?chmod_out.status,
                "chmod of hook, status: "
        );

        let mut mv_cmd = Command::new("mv");
        mv_cmd.arg("pre-commit").arg(".git/hooks/pre-commit");
        let mv_hook_out = mv_cmd
                .current_dir(abs_path_proj)
                .output()
                .context("Failed to run mv pre-commit .git/hooks/pre-commit")?;
        event!(
                Level::DEBUG,
                %mv_hook_out.status,
                "move_pre_commit_hook, status:"
        );

        Ok(())
}

/// gets workind directory and returns as a `String`
#[tracing::instrument]
fn get_current_working_dir() -> anyhow::Result<String> {
        Ok(env::current_dir()?
                .into_os_string()
                .into_string()
                .map_err(|_| io::Error::new(io::ErrorKind::Other, "invalid unicode data in OSString, could not convert to standard string"))?)
}
