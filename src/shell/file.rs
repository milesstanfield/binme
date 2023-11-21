use super::output::check_and_format_output;
use regex::Regex;
use std::process::{Command, Stdio};

pub fn find_files(dir: &str) -> String {
    let output = Command::new("find")
        .arg(dir)
        .arg("-readable")
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute command");

    check_and_format_output(output)
}

pub fn file_extension(file: &str) -> String {
    if file.contains(".") {
        let regex = Regex::new(r"(.*)(\.)").unwrap();
        regex.replace_all(file, "${2}").to_string()
    } else {
        "".to_string()
    }
}
