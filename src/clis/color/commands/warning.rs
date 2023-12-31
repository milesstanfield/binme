use crate::{args::expr, clis::doc::commands::usage, EXE_WORD};
use colored::Colorize;
use std::collections::VecDeque;

pub const PRINT_WARNING_DESCRIPTION: &str = "print a warning message in an appropriate color/style";

pub fn print_warning_cmd(args: &mut VecDeque<String>) {
    if expr::contains_help_arg(args) || args.len() == 0 {
        print_warning_cmd_usage();
    } else if let Some(first_arg) = args.remove(0) {
        print_warning(&first_arg);
    }
}

pub fn print_warning(str: &String) {
    println!("{}", str.yellow().bold());
}

fn print_warning_cmd_usage() {
    let mut args = VecDeque::from([
        format!("{} color print_warning <string>", EXE_WORD),
        "description".to_string(),
        PRINT_WARNING_DESCRIPTION.to_string(),
    ]);
    usage::print_usage_cmd(&mut args);
}
