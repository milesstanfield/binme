use clis::doc::handle_doc_args;
use std::{collections::VecDeque, env};

pub mod clis;

fn main() {
    // skip(1) cuz the first arg is the relative path to the executed binary
    let mut args: VecDeque<String> = env::args().skip(1).collect();

    if let Some(cli) = args.remove(0) {
        match cli.as_str() {
            "doc" => handle_doc_args(&args),
            _ => handle_unknown_cli(&cli),
        }
    } else {
        handle_no_args();
    }
}

fn handle_unknown_cli(arg: &str) {
    println!("todo {:?} is not a known arg", arg);
}

fn handle_no_args() {
    println!("called binme without any args");
}
