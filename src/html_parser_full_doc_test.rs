use std::fs::{read_dir};
use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::fs::File;
use html_parser::*;

fn get_files_in_dir(dir: PathBuf) -> Vec<PathBuf> {
    let mut paths = vec!();
    if dir.is_dir() {
        for entry in read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                let mut nested_paths = get_files_in_dir(path);
                paths.append(&mut nested_paths);
            } else {
                let path_clone = path.clone();
                let file_name = path_clone.file_name().unwrap().to_str().unwrap();
                println!("Filename: {}", file_name);
                if file_name.ends_with("failing.html") {
                    continue;
                }
                paths.push(path);
            }
        }
    }
    return paths;
}

#[test]
fn it_parses_docs() {
    let p = Path::new("examples").to_path_buf();
    let files = get_files_in_dir(p);
    for file in files {
        let mut fh = File::open(&file).unwrap();
        let mut s = String::new();
        fh.read_to_string(&mut s).unwrap();
        let mut warnings = vec!();
        println!("Parsing file: {:?}", &file);
        parse(s, &mut warnings);
        println!("Warnings: {:?}", warnings);
        assert_eq!(0, warnings.len(), "there should be no warnings");
    }
}
