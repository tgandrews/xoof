extern crate getopts;

pub mod dom;
mod entry;
pub mod html_parser;
pub mod parser;

#[cfg(test)]
mod html_parser_test;

fn main() {
    entry::entry();
}
