use super::output;
use regex::Regex;
use std::process::{Command, Stdio};

pub fn find_files(dir: &str) -> String {
    let output = Command::new("find")
        .arg(dir)
        .arg("-readable")
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute command");

    output::check_and_format_output(output)
}

pub fn file_extension(file: &str) -> String {
    if file.contains(".") {
        Regex::new(r"(.*)(\.)")
            .unwrap()
            .replace_all(file, "${2}")
            .to_string()
    } else {
        "".to_string()
    }
}
