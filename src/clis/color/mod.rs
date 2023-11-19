use self::commands::{
    error::{print_error_cmd, PRINT_ERROR_DESCRIPTION},
    sample::{print_sample_cmd, PRINT_SAMPLE_DESCRIPTION},
    success::{print_success_cmd, PRINT_SUCCESS_DESCRIPTION},
    warning::{print_warning_cmd, PRINT_WARNING_DESCRIPTION},
};
use super::doc::commands::usage::print_usage_cmd;
use crate::{args::handling::handle_invalid_arg, EXE_WORD};
use std::collections::VecDeque;

pub mod commands;
pub const COLOR_CLI_DESCRIPTION: &str = "cli for formatting println code in color";

pub fn color_cli(args: &mut VecDeque<String>) {
    if let Some(cmd) = args.remove(0) {
        match cmd.as_str() {
            "print_sample" => print_sample_cmd(args),
            "print_error" => print_error_cmd(args),
            "print_warning" => print_warning_cmd(args),
            "print_success" => print_success_cmd(args),
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
        format!("print_success --- {}", PRINT_SUCCESS_DESCRIPTION),
        format!("print_warning --- {}", PRINT_WARNING_DESCRIPTION),
        format!("print_sample --- {}", PRINT_SAMPLE_DESCRIPTION),
    ]);
    print_usage_cmd(&mut args);
}
