use super::{color::commands::error, doc::commands::usage};
use crate::{args::expr, EXE_WORD};
use regex::Regex;
use std::{collections::VecDeque, process};

pub const GREPX_CLI_DESCRIPTION: &str = "cli for grepping with regex";

pub fn grepx_cli(args: &mut VecDeque<String>) {
    if expr::contains_help_arg(args) {
        print_grepx_cli_help();
    } else {
        match args.len() {
            0 => handle_missing_arg("missing <haystack> <regex> args"),
            1 => handle_missing_arg("missing <regex> arg"),
            _ => grepx_cmd(args),
        }
    }
}

fn grepx_cmd(args: &mut VecDeque<String>) {
    let haystack = &args[0];
    let regex_arg = &args[1];
    let pattern = patternize(regex_arg);
    let reg = Regex::new(pattern.as_str()).unwrap();

    let found_lines: Vec<&str> = haystack
        .split("\\n")
        .filter(|line| reg.is_match(line))
        .collect();

    for found_line in found_lines {
        println!("{found_line}");
    }
}

fn patternize(pattern: &String) -> String {
    if pattern.starts_with('/') {
        pattern[1..(pattern.len() - 2)].to_string()
    } else {
        pattern.to_string()
    }
}

fn handle_missing_arg(msg: &str) {
    error::print_error(&msg.to_string());
    print_grepx_cli_help();
    process::exit(1);
}

fn print_grepx_cli_help() {
    let example1 = r#" grepx "foo\nbar\nbaz" "ba[\w]+""#;
    let example2 = r#" grepx "foo\nbar\nbaz" "/ba[\w]+/""#;

    let mut args = VecDeque::from([
        format!("{} grepx <haystack> <regex>", EXE_WORD),
        "description".to_string(),
        GREPX_CLI_DESCRIPTION.to_string(),
        "examples".to_string(),
        format!("{}{}", EXE_WORD, example1),
        format!("{}{}", EXE_WORD, example2),
    ]);
    usage::print_usage_cmd(&mut args);
}
