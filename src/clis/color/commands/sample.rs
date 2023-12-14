use crate::{args::expr, clis::doc::commands::usage, EXE_WORD};
use colored::Colorize;
use std::collections::VecDeque;

pub const PRINT_SAMPLE_DESCRIPTION: &str = "print a sampling of color combinations and their code";

pub fn print_sample_cmd(args: &mut VecDeque<String>) {
    if expr::contains_help_arg(args) {
        print_sample_cmd_help();
    } else {
        print_sample();
    }
}

fn print_sample_cmd_help() {
    let mut args = VecDeque::from([
        format!("{} color sample", EXE_WORD),
        "description".to_string(),
        PRINT_SAMPLE_DESCRIPTION.to_string(),
    ]);
    usage::print_usage_cmd(&mut args);
}

fn print_sample() {
    let toto = "test";
    println!("{} red()", toto.red());
    println!("{} blue()", toto.blue());

    println!("{} bold()", toto.bold());
    println!("{} red().bold()", toto.red().bold());
    println!("{} yellow().bold()", toto.bold().yellow());
    println!("{} blue().bold()", toto.bold().blue());
    println!("{} green().bold()", toto.bold().green());
    println!(
        "{} blue().bold().underline()",
        toto.blue().bold().underline()
    );
    println!("{} blue().italic()", toto.blue().italic());
    println!("{} green().on_blue()", toto.green().on_blue());
    println!("{} on_magenta().yellow()", toto.on_magenta().yellow());
    println!("{} purple().on_yellow()", toto.purple().on_yellow());
    println!("{} magenta().on_white()", toto.magenta().on_white());
    println!("{} cyan().on_green()", toto.cyan().on_green());
    println!("{} black().on_white()", toto.black().on_white());
    println!("{} green()", toto.green());
    println!("{} yellow()", toto.yellow());
    println!("{} purple()", toto.purple());
    println!("{} magenta()", toto.magenta());
    println!("{} cyan()", toto.cyan());
    println!("{} white()", toto.white());
    println!(
        "{} white().red().blue().green()",
        toto.white().red().blue().green()
    );
    println!("{} truecolor(255, 0, 0)", toto.truecolor(255, 0, 0));
    println!("{} truecolor(255, 255, 0)", toto.truecolor(255, 255, 0));
    println!("{} on_truecolor(0, 80, 80)", toto.on_truecolor(0, 80, 80));
}
