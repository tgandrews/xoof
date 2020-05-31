extern crate getopts;

mod cssom;
mod document;
mod dom;
mod entry;
mod parser;
mod styling;

fn main() {
    entry::entry();
}
