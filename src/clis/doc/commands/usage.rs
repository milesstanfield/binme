use colored::Colorize;
use std::collections::VecDeque;

pub fn usage_cmd(args: &mut VecDeque<String>) {
    if contains_help_arg(args) || args.len() == 0 {
        print_usage_cmd_help();
    } else if let Some(first_arg) = args.remove(0) {
        print_documentation(&first_arg, args);
    }
}

fn contains_help_arg(args: &mut VecDeque<String>) -> bool {
    for arg in args {
        if ["--help", "help", "-h"].contains(&arg.as_str()) {
            return true;
        }
    }
    return false;
}

fn print_documentation(usage: &str, args: &mut VecDeque<String>) {
    let options = "[<options>...]";
    let spacer = "\t\t\t";
    let splitter = " --- ";
    let mut str = format!("Usage: {} {}\n", usage, options).bold().to_string();

    for (i, arg) in args.iter().enumerate() {
        if arg == "description" || arg == "examples" {
            str += &format!("\n{}:\n", arg);
        } else if arg.contains("turn this -------")
            || arg.contains("binme")
            || (i != 0 && args[i - 1] == "description")
        {
            if i != 0 && args[i - 1] == "description" {
                str += &format!("    {}\n", arg.yellow());
            } else {
                str += &format!("    {}\n", arg);
            }
        } else if arg.contains(splitter) {
            str += &format!("    {} {}\n", arg.replace(splitter, spacer), options);
        } else {
            str += &format!("\nValid {}:\n", arg);
        }
    }

    if !str.contains("Valid options:") || str.contains("turn this -------") {
        str += "\nValid options:\n";
    }

    str += "    help, -h, --help\t\tdisplay help info and exit\n";
    str += "\nExit Status:\nReturns 0 unless an invalid input is given or the\nexecution fails";

    println!("{str}");
}

fn usage_cmd_description() -> String {
    return r#"--------------------------------------   |  --------------------------------------
    turn this ----------------------------   |  into this ----------------------------
    --------------------------------------   |  --------------------------------------
                                             |
    binme doc usage "foo <command>"          |  Usage: foo <command> [<options>...]
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
    "#.to_string();
}

fn print_usage_cmd_help() {
    // todo interpolate "binme" from a global constant
    let mut args: VecDeque<String> = VecDeque::from([
        "binme doc usage <arg> [<arg>...]".to_string(),
        "description".to_string(),
        usage_cmd_description(),
        "examples".to_string(),
        r#"binme doc usage "foo <commands>" "commands" "cmd1 --- lorem ipsum""#.to_string(),
        r#"binme doc usage "foo <option>" "options" "--option1 --- lorem ipsum""#.to_string(),
        r#"binme doc usage "foo <arg>" "description" "lorem ipsum dolor sit amet""#.to_string(),
    ]);
    usage_cmd(&mut args);
}
