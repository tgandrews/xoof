use dom::*;
use parser::html_parser;
use std::fs::read_dir;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

#[test]
fn it_parses_element() {
    let node = get_nth_child("<test></test>".to_string(), 0);
    match node.node_type {
        NodeType::Element(e) => assert_eq!(e.tag_name, "test"),
        _ => assert!(false, "Wrong node type"),
    }
}

#[test]
fn it_parses_child() {
    let node = get_nth_child("<test><child></child></test>".to_string(), 0);
    let ref first_child = node.children[0];
    match &first_child.node_type {
        &NodeType::Element(ref e) => assert_eq!(e.tag_name, "child"),
        _ => assert!(false, "Wrong node type"),
    }
}

#[test]
fn it_parses_siblings() {
    let node = get_nth_child(
        "<test><child></child><child2></child2></test>".to_string(),
        0,
    );
    let ref second_child = node.children[1];
    match &second_child.node_type {
        &NodeType::Element(ref e) => assert_eq!(e.tag_name, "child2"),
        _ => assert!(false, "Wrong node type"),
    }
}

#[test]
fn it_parse_text_node() {
    let node = get_nth_child("<h1>hello world</h1>".to_string(), 0);
    let ref first_child = node.children[0];
    match &first_child.node_type {
        &NodeType::Text(ref c) => assert_eq!(c, "hello world"),
        _ => assert!(false, "Wrong node type"),
    }
}

#[test]
fn it_parses_attributes() {
    let node = get_nth_child("<h1 id=\"title\">Hello world</h1>".to_string(), 0);
    match node.node_type {
        NodeType::Element(e) => {
            let id = match e.attributes.get("id") {
                Some(v) => v,
                None => "No id",
            };
            assert_eq!(id, "title");
        }
        _ => assert!(false, "Wrong node type"),
    }
}

#[test]
fn it_parses_attributes_with_quotes() {
    let node = get_nth_child(
        "<a title=\"here is - 'something'\">Hello</a>".to_string(),
        0,
    );
    match node.node_type {
        NodeType::Element(e) => {
            match e.attributes.get("title") {
                Some(v) => assert_eq!(v, "here is - 'something'"),
                None => assert!(false, "No title!"),
            };
        }
        _ => assert!(false, "Wrong node type"),
    }
}

#[test]
fn it_parses_doctype() {
    let node = get_nth_child("<!DOCTYPE html>".to_string(), 0);
    match node.node_type {
        NodeType::DocType(ref e) => assert_eq!(e.version, "html"),
        _ => assert!(false, "Wrong node type"),
    }
}

#[test]
fn it_parses_document_as_doctype_sibling() {
    let node = get_nth_child("<!DOCTYPE html><html></html>".to_string(), 1);
    match node.node_type {
        NodeType::Element(ref e) => assert_eq!(e.tag_name, "html"),
        _ => assert!(false, "Wrong node type"),
    }
}

#[test]
fn it_parses_self_closing_link() {
    let node = get_nth_child("<head><link></head>".to_string(), 0);
    let ref link = node.children[0];
    match &link.node_type {
        &NodeType::Element(ref e) => assert_eq!(e.tag_name, "link"),
        _ => assert!(false, "Wrong node type"),
    }
}

#[test]
fn it_parses_self_closing_meta() {
    let node = get_nth_child("<head><meta></head>".to_string(), 0);
    let ref link = node.children[0];
    match &link.node_type {
        &NodeType::Element(ref e) => assert_eq!(e.tag_name, "meta"),
        _ => assert!(false, "Wrong node type"),
    }
}

#[test]
fn it_does_not_include_next_inside_self_closing() {
    let node = get_nth_child("<meta><link>".to_string(), 1);
    match node.node_type {
        NodeType::Element(e) => assert_eq!(e.tag_name, "link"),
        _ => assert!(false, "Wrong node type"),
    }
}

#[test]
fn it_parses_comments() {
    let node = get_nth_child("<!-- hello world -->".to_string(), 0);
    match node.node_type {
        NodeType::Comment(c) => assert_eq!(c, " hello world "),
        _ => assert!(false, "Wrong node type"),
    }
}

#[test]
fn it_parses_coments_with_dashes() {
    let node = get_nth_child("<!-- hello - world -->".to_string(), 0);
    match node.node_type {
        NodeType::Comment(c) => assert_eq!(c, " hello - world "),
        _ => assert!(false, "Wrong node type"),
    }
}

#[test]
fn it_parses_cdata() {
    let node = get_nth_child("<![CDATA[<h1>Hello world]]>".to_string(), 0);
    match node.node_type {
        NodeType::CData(c) => assert_eq!(c, "<h1>Hello world"),
        _ => assert!(false, "Wrong node type"),
    }
}

#[test]
fn it_parses_self_closing_tags() {
    let node = get_nth_child("<h1 /><p>Hello</p>".to_string(), 0);
    match node.node_type {
        NodeType::Element(e) => assert_eq!(e.tag_name, "h1"),
        _ => assert!(false, "Wrong node type"),
    }
}

#[test]
fn it_closes_children_with_parent() {
    let node = get_nth_child("<ul><li>Hello</ul>".to_string(), 0);
    let ref child = node.children[0];
    match &child.node_type {
        &NodeType::Element(ref e) => assert_eq!(e.tag_name, "li"),
        _ => assert!(false, "Wrong node type"),
    }
}

#[test]
fn it_parser_multi_line_self_closing_elements() {
    let node = get_nth_child(
        "<div
    a='1'
    />"
        .to_string(),
        0,
    );
    match node.node_type {
        NodeType::Element(ref e) => assert_eq!(e.tag_name, "div"),
        _ => assert!(false, "Wrong node type"),
    }
}

fn get_nth_child(text: String, pos: usize) -> Node {
    let mut warnings = vec![];
    let nodes = html_parser::parse(text, &mut warnings);
    for warn in &warnings {
        println!("Warn: {}", warn)
    }
    assert_eq!(warnings.len(), 0, "No warnings expected");
    let node = nodes[pos].clone();
    return node;
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

#[test]
fn it_parses_docs() {
    let p = Path::new("examples").to_path_buf();
    let files = get_files_in_dir(p);
    for file in files {
        if !file.to_str().unwrap().ends_with(".html") {
            continue;
        }

        let mut fh = File::open(&file).unwrap();
        let mut s = String::new();
        fh.read_to_string(&mut s).unwrap();
        let mut warnings = vec![];
        println!("Parsing file: {:?}", &file);
        html_parser::parse(s, &mut warnings);
        for warn in &warnings {
            println!("  {}", warn);
        }
        assert_eq!(0, warnings.len(), "there should be no warnings");
    }
}
