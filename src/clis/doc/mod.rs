use self::commands::usage::usage_cmd;
use crate::{args::handling::handle_invalid_arg, EXE_WORD};
use std::collections::VecDeque;

pub mod commands;

pub const DOC_CLI_DESCRIPTION: &str = "cli for displaying documentation";

pub fn doc_cli(args: &mut VecDeque<String>) {
    if let Some(cmd) = args.remove(0) {
        match cmd.as_str() {
            "usage" => usage_cmd(args),
            "help" | "--help" | "-h" => print_doc_cli_usage(),
            _ => handle_invalid_arg("command", &cmd, print_doc_cli_usage),
        }
    } else {
        print_doc_cli_usage();
    }
}

fn print_doc_cli_usage() {
    // todo add description?
    let mut args: VecDeque<String> = VecDeque::from([
        format!("{} doc <command>", EXE_WORD),
        "description".to_string(),
        DOC_CLI_DESCRIPTION.to_string(),
        "commands".to_string(),
        "usage --- display usage for this cli".to_string(),
    ]);
    usage_cmd(&mut args);
}
