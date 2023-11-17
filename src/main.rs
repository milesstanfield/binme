use clis::doc::doc_cli;
use color::{print_warning, sample};
use std::{collections::VecDeque, env};

pub mod clis;
pub mod color;

fn main() {
    let mut args: VecDeque<String> = env::args().skip(1).collect();

    if let Some(cli) = args.remove(0) {
        match cli.as_str() {
            "doc" => doc_cli(&args),
            _ => handle_unknown_cli(&cli),
        }
    } else {
        handle_no_args();
    }
}

fn handle_unknown_cli(arg: &str) {
    print_warning(&format!("todo is not a known arg {}", arg));
}

fn handle_no_args() {
    println!("called binme without any args");
}
