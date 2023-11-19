use self::commands::{
    error::{print_error_cmd, PRINT_ERROR_DESCRIPTION},
    sample::{sample_cmd, PRINT_SAMPLE_DESCRIPTION},
};
use super::doc::commands::usage::usage_cmd;
use crate::{args::handling::handle_invalid_arg, EXE_WORD};
use std::collections::VecDeque;

pub mod commands;

pub const COLOR_CLI_DESCRIPTION: &str = "cli for formatting println code in color";

pub fn color_cli(args: &mut VecDeque<String>) {
    if let Some(cmd) = args.remove(0) {
        match cmd.as_str() {
            "sample" => sample_cmd(args),
            "print_error" => print_error_cmd(args),
            // "print_warning" => print_warning_cmd(args),
            // "print_success" => print_success_cmd(args),
            "help" | "--help" | "-h" => print_color_cli_usage(),
            _ => handle_invalid_arg("command", &cmd, print_color_cli_usage),
        }
    } else {
        print_color_cli_usage();
    }
}

fn print_color_cli_usage() {
    let mut args: VecDeque<String> = VecDeque::from([
        format!("{} color <command>", EXE_WORD),
        "description".to_string(),
        COLOR_CLI_DESCRIPTION.to_string(),
        "commands".to_string(),
        format!("print_error --- {}", PRINT_ERROR_DESCRIPTION),
        format!("sample      --- {}", PRINT_SAMPLE_DESCRIPTION),
    ]);
    usage_cmd(&mut args); // todo should we rename this since its shared to `print_usage` ?
}

// todo
// pub fn print_success(str: String) {
//     println!("{}", str.green().bold());
// }

// pub fn print_warning(str: String) {
//     println!("{}", str.yellow().bold());
// }
