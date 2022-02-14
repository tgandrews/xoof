extern crate getopts;

mod cssom;
mod document;
mod dom;
mod entry;
mod layout;
mod parser;
mod styling;

fn main() {
    entry::entry();
}
