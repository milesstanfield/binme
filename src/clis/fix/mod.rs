use self::commands::format::format_cmd;
use super::doc::commands::usage::usage_cmd;
use crate::color::print_error;
use std::{collections::VecDeque, process::exit};

pub mod commands;

pub fn fix_cli(args: &mut VecDeque<String>) {
    if let Some(cmd) = args.remove(0) {
        match cmd.as_str() {
            "format" => format_cmd(args),
            "help" | "--help" | "-h" => print_fix_cli_help(),
            _ => handle_invalid_cmd(&cmd),
        }
    } else {
        print_fix_cli_help();
    }
}

fn handle_invalid_cmd(cmd: &str) {
    print_error(format!("invalid <command> `{}`", cmd));
    print_fix_cli_help();
    exit(1);
}

fn print_fix_cli_help() {
    let mut args: VecDeque<String> = VecDeque::from([
        "binme fix <command>".to_string(),
        "commands".to_string(),
        "format --- todo".to_string(),
    ]);
    usage_cmd(&mut args);
}
