use crate::{args::expr::contains_help_arg, EXE_WORD};
use colored::Colorize;
use regex::Regex;
use std::collections::VecDeque;

pub fn print_usage_cmd(args: &mut VecDeque<String>) {
    if contains_help_arg(args) || args.len() == 0 {
        print_usage_cmd_usage();
    } else if let Some(first_arg) = args.remove(0) {
        print_documentation(&first_arg, args);
    }
}

fn print_documentation(usage: &str, args: &VecDeque<String>) {
    let options = "[<options>...]";
    let help_options = "help, -h, --help";
    let spacer = "\t\t";
    let indenter = "     ";
    let splitter = " --- ";
    let mut str = format!("Usage: {} {}\n", usage, options).bold().to_string();

    for (i, arg) in args.iter().enumerate() {
        if arg == "description" || arg == "examples" {
            str += &format!("\n{}:\n", arg);
        } else if arg.contains(EXE_WORD) || last_arg(i, args) == "description" {
            if last_arg(i, args) == "description" {
                str += &format!("{}{}\n", indenter, arg.yellow());
            } else {
                str += &format!("{}{}\n", indenter, arg);
            }
        } else if arg.contains(splitter) {
            let diff_spaces = arg_diff_spaces(splitter, arg, help_options);
            let replacement = format!("{}{}", diff_spaces, spacer);
            str += &format!("{}{}\n", indenter, arg.replace(splitter, &replacement));
        } else {
            str += &format!("\nValid {}:\n", arg);
        }
    }

    if !str.contains("Valid options:") || str.contains("turn this -------") {
        str += "\nValid options:\n";
    }

    let description = "display help info and exit";
    str += format!("{}{}{}{}\n", indenter, help_options, spacer, description).as_str();
    str += "\nExit Status:\nReturns 0 unless an invalid input is given or the\nexecution fails";

    println!("{}", str);
}

fn last_arg(i: usize, args: &VecDeque<String>) -> &str {
    match i {
        0 => "",
        _ => args[i - 1].as_str(),
    }
}

fn arg_diff_spaces(splitter: &str, arg: &str, help_options: &str) -> String {
    let regex = Regex::new(format!("{}.*", splitter).as_str()).expect("regex empty?");
    let pre_split_arg = regex.replace_all(arg, "");
    let len_diff = pre_split_arg.len().abs_diff(help_options.len());
    " ".repeat(len_diff)
}

fn usage_description() -> String {
    format!(
        r#"--------------------------------------   |  --------------------------------------
     turn this ----------------------------   |  into this ----------------------------
     --------------------------------------   |  --------------------------------------
                                              |
     {} doc usage "foo <command>"          |  Usage: foo <command> [<options>...]
     "commands"                               |
         "bar --- does some baR thing"        |  Valid commands:
         "baz --- does some baZ thing"        |      bar                   miles terminal [<options>...]
                                              |      baz                   miles dock [<options>...]
     --------------------------------------   |
     --------------------------------------   |  Valid options:
     --------------------------------------   |      help, -h, --help      display help info and exit
                                              |
                                              |  Exit Status:
                                              |  Returns 0 unless an invalid input is given or the
                                              |  execution fails
                                              |
                                              |  --------------------------------------
                                              |  --------------------------------------
    "#,
        EXE_WORD
    )
}

fn print_usage_cmd_usage() {
    let example1 = r#" doc usage "foo <commands>" "commands" "cmd1 --- lorem ipsum""#;
    let example2 = r#" doc usage "foo <option>" "options" "--option1 --- lorem ipsum""#;
    let example3 = r#" doc usage "foo <arg>" "description" "lorem ipsum dolor sit amet""#;

    let mut args: VecDeque<String> = VecDeque::from([
        format!("{} doc usage <arg> [<arg>...]", EXE_WORD),
        "description".to_string(),
        usage_description(),
        "examples".to_string(),
        format!("{} {}", EXE_WORD, example1),
        format!("{} {}", EXE_WORD, example2),
        format!("{} {}", EXE_WORD, example3),
    ]);
    print_usage_cmd(&mut args);
}
