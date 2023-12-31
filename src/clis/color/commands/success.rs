use crate::{args::expr, clis::doc::commands::usage, EXE_WORD};
use colored::Colorize;
use std::collections::VecDeque;

pub const PRINT_SUCCESS_DESCRIPTION: &str = "print a success message in an appropriate color/style";

pub fn print_success_cmd(args: &mut VecDeque<String>) {
    if expr::contains_help_arg(args) || args.len() == 0 {
        print_success_cmd_usage();
    } else if let Some(first_arg) = args.remove(0) {
        print_success(&first_arg);
    }
}

pub fn print_success(str: &String) {
    println!("{}", str.green().bold());
}

fn print_success_cmd_usage() {
    let mut args = VecDeque::from([
        format!("{} color print_success <string>", EXE_WORD),
        "description".to_string(),
        PRINT_SUCCESS_DESCRIPTION.to_string(),
    ]);
    usage::print_usage_cmd(&mut args);
}
