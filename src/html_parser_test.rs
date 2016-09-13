use html_parser::*;
use dom::*;

#[test]
fn it_parses_element() {
    let node = parse("<test></test>".to_string());
    match node.node_type {
        NodeType::Element(e) => assert_eq!(e.tag_name, "test"),
        _ => assert!(false, "Wrong node type")
    }
}

#[test]
fn it_parses_child() {
    let node = parse("<test><child></child></test>".to_string());
    let ref first_child = node.children[0];
    match &first_child.node_type {
        &NodeType::Element(ref e) => assert_eq!(e.tag_name, "child"),
        _ => assert!(false, "Wrong node type")
    }
}

#[test]
fn it_parses_siblings() {
    let node = parse("<test><child></child><child2></child2></test>".to_string());
    let ref second_child = node.children[1];
    match &second_child.node_type {
        &NodeType::Element(ref e) => assert_eq!(e.tag_name, "child2"),
        _ => assert!(false, "Wrong node type")
    }
}

#[test]
fn it_parse_text_node() {
    let node = parse("<h1>hello world</h1>".to_string());
    let ref first_child = node.children[0];
    match &first_child.node_type {
        &NodeType::Text(ref c) => assert_eq!(c, "hello world"),
        _ => assert!(false, "Wrong node type")
    }
}

#[test]
fn it_parses_attributes() {
    let node = parse("<h1 id=\"title\">Hello world</h1>".to_string());
    match node.node_type {
        NodeType::Element(e) => {
            let id = match e.attributes.get("id") {
                Some(v) => v,
                None => "No id"
            };
            assert_eq!(id, "title");
        },
        _ => assert!(false, "Wrong node type")
    }
}

#[test]
fn it_parses_doctype() {
    let node = parse("<!DOCTYPE html>".to_string());
    match node.node_type {
        NodeType::DocType(e) => assert_eq!(e.version, "html"),
        _ => assert!(false, "Wrong node type")
    }
}
