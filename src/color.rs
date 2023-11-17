use colored::Colorize;

pub fn print_bold(str: &String) {
    println!("{}", str.bold());
}

pub fn print_warning(str: &String) {
    println!("{}", str.yellow());
}

pub fn print_error(str: &String) {
    println!("{}", str.truecolor(255, 0, 0));
}

pub fn print_success(str: &String) {
    println!("{}", str.green().bold());
}

pub fn sample() {
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
