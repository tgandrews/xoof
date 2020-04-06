use getopts::Options;
use std::env;
use std::fs::File;
use std::io::Read;

use html_parser;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn show_error(error: &str) {
    println!("Error: {}", error);
    println!("You can pass --help for more info");
}

pub fn entry() {
    println!("Welcome to Xoof");

    let mut opts = Options::new();
    opts.optopt("h", "html", "set the html file to parse", "[FILE]");
    opts.optflag("", "help", "print this help menu");

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            show_error(format!("{}", f).as_str());
            return;
        }
    };
    if matches.opt_present("help") {
        print_usage(&program, opts);
        return;
    }

    let html_file_path = match matches.opt_str("h") {
        Some(p) => p,
        None => {
            show_error("Missing html file path");
            return;
        }
    };

    println!("File path: {}", html_file_path);
    let html = read_source(html_file_path);
    let mut warnings = vec![];
    let dom_tree = html_parser::parse(html, &mut warnings);
    println!("DOM Tree:");
    for node in &dom_tree {
        println!("{}", node);
    }
    println!("Warnings:");
    for warn in &warnings {
        println!("{}", warn);
    }
}

fn read_source(file_path: String) -> String {
    let mut buffer = String::new();
    File::open(file_path)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();
    buffer
}
