use std::process::exit;

use crate::clis::color::commands::error;

pub fn handle_invalid_arg(intended_for: &str, arg: &str, print_help: fn()) {
    error::print_error(&format!("invalid <{}> `{}`", intended_for, arg));
    print_help();
    exit(1);
}
