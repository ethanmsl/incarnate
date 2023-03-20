/// Collection of actions that use the shell directly.
///
/// # Examples
/// - the `pre-commit` hook is created as a non-executable file
/// - running `git init` helps ensure users normal git settings are respected
use std::env;
use std::process::Command;

pub fn git_init() {
    let cwd = get_current_working_dir();
    println!("Current working directory: {}", cwd);

    let mut git_cmd = Command::new("git");
    git_cmd.arg("log");
    let output = git_cmd.output().unwrap();
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}

fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string(),
    }
}
