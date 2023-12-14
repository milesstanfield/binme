use std::collections::VecDeque;

pub fn contains_help_arg(args: &mut VecDeque<String>) -> bool {
    args.iter()
        .any(|arg| ["--help", "help", "-h"].contains(&arg.as_str()))
}
