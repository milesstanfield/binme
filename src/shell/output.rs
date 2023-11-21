use crate::clis::color::commands::error::print_error_cmd;
use std::{
    collections::VecDeque,
    process::{exit, Output},
};

pub fn check_and_format_output(output: Output) -> String {
    if let Some(1) = output.status.code() {
        match String::from_utf8(output.stderr) {
            Ok(stderr) => check_stderr_string(&stderr),
            Err(e) => panic!("invalid utf8 sequence {}", e),
        }
    } else {
        match String::from_utf8(output.stdout) {
            Ok(stdout) => stdout,
            Err(e) => panic!("invalid utf8 sequence {}", e),
        }
    }
}

fn check_stderr_string(stderr: &str) -> String {
    if stderr.is_empty() {
        "".to_string()
    } else {
        print_error_cmd(&mut VecDeque::from([stderr.to_string()]));
        exit(1)
    }
}
