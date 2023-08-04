use std::process::Command;

pub fn add_all() {
  Command::new("git")
    .args(["add", "."])
    .output()
    .expect("failed to execute process");
}