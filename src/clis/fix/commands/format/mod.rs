use self::rust::{format_rust_file, format_rust_project};
use crate::shell::dir::current_dir;
use crate::shell::file::{file_extension, find_files};
use crate::{args::expr::contains_help_arg, clis::doc::commands::usage::print_usage_cmd, EXE_WORD};
use std::collections::VecDeque;

pub mod rust;
pub const FORMAT_DESCRIPTION: &str = "fix file formatting issues";

pub fn format_cmd(args: &mut VecDeque<String>) {
    if contains_help_arg(args) {
        format_cmd_usage();
    } else if args.len() == 0 {
        format_files_in_dir(current_dir().as_str());
    } else if let Some(first_arg) = args.remove(0) {
        format_files_in_dir(&first_arg.as_str());
    }
}

fn format_files_in_dir(dir: &str) {
    let files = find_files(dir);

    if files.contains(&r".rs".to_string()) {
        format_rust_project();
    }

    for file in files.lines() {
        match file_extension(file).as_str() {
            ".rs" => format_rust_file(file),
            _ => (),
        }
    }
}

fn format_cmd_usage() {
    let mut args: VecDeque<String> = VecDeque::from([
        format!("{} fix format <dir>", EXE_WORD),
        "description".to_string(),
        FORMAT_DESCRIPTION.to_string(),
    ]);
    print_usage_cmd(&mut args);
}
