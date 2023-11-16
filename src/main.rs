use std::env;

const VALID_CLIS: [&str; 1] = ["doc"];

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.len() {
        0 => handle_no_args(),
        _ => handle_first_arg(args[0].as_str()),
    }
}

fn handle_first_arg(first_arg: &str) {
    match VALID_CLIS.contains(&first_arg) {
        true => handle_valid_cli(&first_arg),
        false => handle_unknown_cli(&first_arg),
    }
}

fn handle_valid_cli(cli: &str) {
    println!("todo doc was first arg {:?}", cli)
}

fn handle_unknown_cli(cli: &str) {
    println!("todo unknown cli {:?}", cli)
}

fn handle_no_args() {
    println!("called binme without any args")
}
