use crate::clis::doc::commands::usage::usage_cmd;
use args::handling::handle_invalid_arg;
use clis::{color::color_cli, doc::doc_cli, fix::fix_cli};
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
        "doc --- cli for displaying documentation".to_string(),
        "fix --- cli for fixing formatting issues".to_string(),
        "color --- cli for formatting println code in color".to_string(),
    ]);
    usage_cmd(&mut args);
}
