use cssom::*;
use parser::css_parser::*;
use std::fs::read_dir;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

#[test]
fn it_parses_element_selector_with_attribute() {
    let style_sheet = parse_css("div { height: 100px; }".to_string());
    let rule: &Rule = style_sheet.rules.get(0).unwrap();
    let selector: &Selector = rule.selectors.get(0).unwrap();
    match &selector.selector_type {
        SelectorType::SimpleSelector(s) => {
            assert_eq!(s.tag_name.clone().unwrap(), String::from("div"));
        }
    };
    let declaration: &Declaration = rule.declarations.get(0).unwrap();
    assert_eq!(declaration.name, String::from("height"));
    match declaration.value {
        Value::Length(amount, unit) => {
            assert_eq!(amount, 100_f32);
            let unit_result = match unit {
                Unit::Px => "px",
            };
            assert_eq!(unit_result, "px")
        }
    };
}

#[test]
fn it_parses_multiple_attributes() {
    let style_sheet = parse_css("span { height: 100px; width: 50px; }".to_string());
    let rule: &Rule = style_sheet.rules.get(0).unwrap();
    let selector: &Selector = rule.selectors.get(0).unwrap();
    match &selector.selector_type {
        SelectorType::SimpleSelector(s) => {
            assert_eq!(s.tag_name.clone().unwrap(), String::from("span"));
        }
    }
    let height_declaration: &Declaration = rule.declarations.get(0).unwrap();
    assert_eq!(height_declaration.name, String::from("height"));
    match height_declaration.value {
        Value::Length(amount, unit) => {
            assert_eq!(amount, 100_f32);
            let unit_result = match unit {
                Unit::Px => "px",
            };
            assert_eq!(unit_result, "px")
        }
    }

    let width_declaration: &Declaration = rule.declarations.get(1).unwrap();
    assert_eq!(width_declaration.name, String::from("width"));
    match width_declaration.value {
        Value::Length(amount, unit) => {
            assert_eq!(amount, 50_f32);
            let unit_result = match unit {
                Unit::Px => "px",
            };
            assert_eq!(unit_result, "px")
        }
    }
}
#[test]
fn it_parse_example_stylesheets() {
    let p = Path::new("examples").to_path_buf();
    let files = get_files_in_dir(p);
    for file in files {
        if !file.to_str().unwrap().ends_with(".css") {
            continue;
        }

        let mut fh = File::open(&file).unwrap();
        let mut s = String::new();
        fh.read_to_string(&mut s).unwrap();
        let mut warnings = vec![];
        println!("Parsing file: {:?}", &file);
        parse(s, &mut warnings);
        for warn in &warnings {
            println!("  {}", warn);
        }
        assert_eq!(0, warnings.len(), "there should be no warnings");
    }
}

fn get_files_in_dir(dir: PathBuf) -> Vec<PathBuf> {
    let mut paths = vec![];
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
                if file_name.ends_with("failing.html") {
                    continue;
                }
                paths.push(path);
            }
        }
    }
    return paths;
}

fn parse_css(style: String) -> StyleSheet {
    let mut warnings = vec![];
    let style_sheet = parse(style, &mut warnings);
    for warn in &warnings {
        println!("Warn: {}", warn)
    }
    assert_eq!(warnings.len(), 0, "No warnings expected");
    return style_sheet.clone();
}
