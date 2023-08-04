use std::process::Command;

pub fn push() {
  Command::new("git")
    .arg("push")
    .output()
    .expect("failed to execute process");
}