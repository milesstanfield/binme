use crate::clis::doc::commands::usage::usage_cmd;
use clis::{doc::doc_cli, fix::fix_cli};
use color::print_error;
use std::{collections::VecDeque, env, process::exit};

pub mod clis;
pub mod color;

fn main() {
    let mut args: VecDeque<String> = env::args().skip(1).collect();

    if let Some(cli) = args.remove(0) {
        match cli.as_str() {
            "doc" => doc_cli(&mut args),
            "fix" => fix_cli(&mut args),
            _ => handle_invalid_cli(&cli),
        }
    } else {
        print_cli_help();
    }
}

fn handle_invalid_cli(cli: &str) {
    print_error(format!("invalid <cli> `{}`", cli));
    print_cli_help();
    exit(1);
}

fn print_cli_help() {
    let mut args: VecDeque<String> = VecDeque::from([
        "binme <cli>".to_string(),
        "clis".to_string(),
        "doc --- cli for displaying documentation".to_string(),
        "fix --- cli for fixing formatting issues".to_string(),
        "examples".to_string(),
    ]);
    usage_cmd(&mut args);
}
