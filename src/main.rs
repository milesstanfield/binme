use crate::clis::doc::commands::usage::print_usage_cmd;
use args::handling::handle_invalid_arg;
use clis::{
    color::{color_cli, COLOR_CLI_DESCRIPTION},
    doc::{doc_cli, DOC_CLI_DESCRIPTION},
    fix::{fix_cli, FIX_CLI_DESCRIPTION},
};
use std::{collections::VecDeque, env};

pub mod args;
pub mod clis;

pub static EXE_WORD: &str = "binme";

fn main() {
    let mut args: VecDeque<String> = env::args().skip(1).collect();

    if let Some(cli) = args.remove(0) {
        match cli.as_str() {
            "doc" => doc_cli(&mut args),
            "fix" => fix_cli(&mut args),
            "color" => color_cli(&mut args),
            "help" | "--help" | "-h" => print_cli_usage(),
            _ => handle_invalid_arg("cli", &cli, print_cli_usage),
        }
    } else {
        print_cli_usage();
    }
}

fn print_cli_usage() {
    let mut args: VecDeque<String> = VecDeque::from([
        format!("{} <cli>", EXE_WORD).to_string(),
        "clis".to_string(),
        format!("doc --- {}", DOC_CLI_DESCRIPTION).to_string(),
        format!("fix --- {}", FIX_CLI_DESCRIPTION).to_string(),
        format!("color --- {}", COLOR_CLI_DESCRIPTION).to_string(),
    ]);
    print_usage_cmd(&mut args);
}
