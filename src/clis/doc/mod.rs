use self::commands::usage::usage_cmd;
use colored::Colorize;
use std::{collections::VecDeque, process::exit};

pub mod commands;

pub fn doc_cli(args: &mut VecDeque<String>) {
    if let Some(cmd) = args.remove(0) {
        match cmd.as_str() {
            "usage" => usage_cmd(args),
            "help" | "--help" | "-h" => print_doc_cli_help(),
            _ => handle_invalid_cmd(&cmd),
        }
    } else {
        print_doc_cli_help();
    }
}

fn handle_invalid_cmd(cmd: &str) {
    println!("{}", &format!("invalid <command> '{}'", cmd).red().bold());
    print_doc_cli_help();
    exit(1);
}

fn print_doc_cli_help() {
    let mut args: VecDeque<String> = VecDeque::from([
        "binme doc <command>".to_string(),
        "commands".to_string(),
        "usage --- display usage for this cli".to_string(),
        "examples".to_string(),
    ]);
    usage_cmd(&mut args);
}
