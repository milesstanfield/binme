use std::collections::VecDeque;

pub fn contains_help_arg(args: &mut VecDeque<String>) -> bool {
    for arg in args {
        if ["--help", "help", "-h"].contains(&arg.as_str()) {
            return true;
        }
    }
    false
}
