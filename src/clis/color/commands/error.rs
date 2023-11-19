use crate::{args::expr::contains_help_arg, clis::doc::commands::usage::print_usage_cmd, EXE_WORD};
use colored::Colorize;
use std::collections::VecDeque;

pub const PRINT_ERROR_DESCRIPTION: &str = "print an error message in an appropriate color/style";

pub fn print_error_cmd(args: &mut VecDeque<String>) {
    if contains_help_arg(args) || args.len() == 0 {
        print_error_cmd_usage();
    } else if let Some(first_arg) = args.remove(0) {
        print_error(&first_arg);
    }
}

pub fn print_error(str: &String) {
    println!("{}: {}", "error".red().bold(), str);
}

fn print_error_cmd_usage() {
    let mut args: VecDeque<String> = VecDeque::from([
        format!("{} color print_error <string>", EXE_WORD),
        "description".to_string(),
        PRINT_ERROR_DESCRIPTION.to_string(),
    ]);
    print_usage_cmd(&mut args);
}
