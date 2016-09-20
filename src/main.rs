extern crate getopts;

pub mod dom;
pub mod html_parser;
mod entry;

#[cfg(test)]
mod html_parser_test;

fn main() {
    entry::entry();
}
