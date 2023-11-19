use self::commands::format::format_cmd;
use super::doc::commands::usage::usage_cmd;
use crate::{args::handling::handle_invalid_arg, EXE_WORD};
use std::collections::VecDeque;

pub mod commands;

pub fn fix_cli(args: &mut VecDeque<String>) {
    if let Some(cmd) = args.remove(0) {
        match cmd.as_str() {
            "format" => format_cmd(args),
            "help" | "--help" | "-h" => print_fix_cli_usage(),
            _ => handle_invalid_arg("command", &cmd, print_fix_cli_usage),
        }
    } else {
        print_fix_cli_usage();
    }
}

fn print_fix_cli_usage() {
    // todo add description?
    let mut args: VecDeque<String> = VecDeque::from([
        format!("{} fix <command>", EXE_WORD),
        "commands".to_string(),
        "format --- todo".to_string(),
    ]);
    usage_cmd(&mut args);
}
