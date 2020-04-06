extern crate getopts;

pub mod dom;
mod entry;
pub mod html_parser;

#[cfg(test)]
mod html_parser_test;

fn main() {
    entry::entry();
}
