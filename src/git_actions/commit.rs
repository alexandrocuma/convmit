use std::process::Command;

pub fn commit(message: &str) {
  Command::new("git")
    .args(["commit", "-m", message])
    .output()
    .expect("failed to execute process");
}