use console::Style;
use std::process::Command;

pub fn commit(message: &str) {
    let red = Style::new().red();
    let green = Style::new().green();

    let output = Command::new("git")
        .args(["commit", "-m", message])
        .output()
        .unwrap();

    if !output.stdout.is_empty() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("\n{}", green.apply_to(stdout));
    }

    if !output.stdout.is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("\n{}", red.apply_to(stderr));
    }
}