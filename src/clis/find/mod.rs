use regex::Regex;

use super::{color::commands::error, doc::commands::usage};
use crate::{args::expr, shell::file, EXE_WORD};
use std::{collections::VecDeque, process};

pub const FIND_CLI_DESCRIPTION: &str = "cli for finding system files/folders";

pub fn find_cli(args: &mut VecDeque<String>) {
    if expr::contains_help_arg(args) {
        print_find_cli_help();
    } else {
        match args.len() {
            0 => handle_missing_arg("missing <dir> <pattern> args"),
            1 => handle_missing_arg("missing <pattern> arg"),
            _ => find_cmd(args),
        }
    }
}

fn find_cmd(args: &mut VecDeque<String>) {
    let dir = &args[0];
    let pattern_arg = &args[1];

    let (is_regex, formatted_pattern) = patternize(pattern_arg);
    let mut reg = Regex::new(r"").unwrap();
    if is_regex {
        reg = Regex::new(formatted_pattern.as_str()).unwrap();
    }

    let findings = file::find_files(dir);
    let found_files: Vec<&str> = findings
        .split("\n")
        .into_iter()
        .filter(|line| {
            if is_regex {
                reg.is_match(line)
            } else {
                line.contains(formatted_pattern.as_str())
            }
        })
        .collect();

    for found_file in found_files {
        println!("{found_file}");
    }
}

fn patternize(pattern: &String) -> (bool, String) {
    if pattern.starts_with('/') {
        return (true, pattern[1..(pattern.len() - 2)].to_string());
    }
    (false, pattern.to_string())
}

fn handle_missing_arg(msg: &str) {
    error::print_error(&msg.to_string());
    print_find_cli_help();
    process::exit(1);
}

fn print_find_cli_help() {
    let mut args = VecDeque::from([
        format!("{} find <dir> <pattern>", EXE_WORD),
        "description".to_string(),
        FIND_CLI_DESCRIPTION.to_string(),
    ]);
    usage::print_usage_cmd(&mut args);
}
