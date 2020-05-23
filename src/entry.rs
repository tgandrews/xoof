use getopts::Options;
use std::env;
use std::fs::File;
use std::io::Read;

use document;

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
    opts.optopt("c", "css", "set the css file to parse", "[FILE]");
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
    let css_file_path = match matches.opt_str("c") {
        Some(p) => p,
        None => {
            show_error("Missing css file path");
            return;
        }
    };

    let html = read_source(html_file_path);
    let css = read_source(css_file_path);

    let document = document::create_document(html, css);
    println!("DOM Tree:");
    for node in document.dom {
        println!("{}", node);
    }
    println!("CSS:");
    for style_rule in document.style_sheet.rules {
        println!("{:#?}", style_rule);
    }
    println!("Warnings:");
    for warn in document.warnings {
        println!("{}", warn);
    }
}

fn read_source(file_path: String) -> String {
    println!("File path: {}", file_path);
    let mut buffer = String::new();
    File::open(file_path)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();
    buffer
}
