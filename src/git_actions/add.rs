use std::process::Command;

pub fn add_all() {
  Command::new("git")
    .args(["add", "."])
    .output()
    .expect("failed to execute process");
}

pub fn add_file(filename: &str) {
  Command::new("git")
    .args(["add", filename])
    .output()
    .expect("failed to execute process");
}