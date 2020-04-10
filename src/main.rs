extern crate getopts;

pub mod css_parser;
pub mod dom;
mod entry;
pub mod html_parser;
pub mod parser;

#[cfg(test)]
mod css_parser_test;
#[cfg(test)]
mod html_parser_test;

fn main() {
    entry::entry();
}
