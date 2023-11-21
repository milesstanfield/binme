use crate::shell::output::check_and_format_output;
use std::process::{Command, Stdio};

// remove blank lines, order imports appropriately
pub fn format_rust_file(file: &str) {
    let output = Command::new("rustfmt")
        .arg(file)
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute command");

    check_and_format_output(output);
}

// remove unused imports
pub fn format_rust_project() {
    let output = Command::new("cargo")
        .arg("fix")
        .arg("--allow-dirty")
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute command");

    check_and_format_output(output);
}
