/// Collection of actions that use the shell directly.
///
/// # Examples
/// - the `pre-commit` hook is created as a non-executable file
/// - running `git init` helps ensure users normal git settings are respected
use std::process::Command;

fn _place_holder() {
    let _some_thing = Command::new("brew")
        .arg("list")
        .output()
        .expect("failed to execute `brew list` capture");
}
