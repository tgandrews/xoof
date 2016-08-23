extern crate getopts;

use getopts::Options;
use std::env;

pub mod dom;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn show_error(error: &str) {
    println!("Error: {}", error);
    println!("You can pass --help for more info");
}

fn main() {
    println!("Welcome to Xoof");

    let mut opts = Options::new();
    opts.optopt("h", "html", "set the html file to parse", "FILE NAME");
    opts.optflag("", "help", "print this help menu");

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { println!("{}", f); return }
    };
    if matches.opt_present("help") {
        print_usage(&program, opts);
        return;
    }

    let html_file_path = match matches.opt_str("h") {
        Some(p) => p,
        None => { show_error("Missing html file path"); return }
    };

    println!("File path: {}", html_file_path);
}
