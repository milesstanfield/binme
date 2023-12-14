use self::commands::format::{format_cmd, FORMAT_DESCRIPTION};
use super::doc::commands::usage::print_usage_cmd;
use crate::{args::handling::handle_invalid_arg, EXE_WORD};
use std::collections::VecDeque;

pub mod commands;

pub const FIX_CLI_DESCRIPTION: &str = "cli for fixing common issues";

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
    let mut args = VecDeque::from([
        format!("{} fix <command>", EXE_WORD),
        "description".to_string(),
        FIX_CLI_DESCRIPTION.to_string(),
        "commands".to_string(),
        format!("format --- {}", FORMAT_DESCRIPTION),
    ]);
    print_usage_cmd(&mut args);
}
