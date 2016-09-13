use html_parser::*;
use dom::*;

#[test]
fn it_parses_element() {
    let ref node = parse("<test></test>".to_string())[0];
    match &node.node_type {
        &NodeType::Element(ref e) => assert_eq!(e.tag_name, "test"),
        _ => assert!(false, "Wrong node type")
    }
}

#[test]
fn it_parses_child() {
    let ref node = parse("<test><child></child></test>".to_string())[0];
    let ref first_child = node.children[0];
    match &first_child.node_type {
        &NodeType::Element(ref e) => assert_eq!(e.tag_name, "child"),
        _ => assert!(false, "Wrong node type")
    }
}

#[test]
fn it_parses_siblings() {
    let ref node = parse("<test><child></child><child2></child2></test>".to_string())[0];
    let ref second_child = node.children[1];
    match &second_child.node_type {
        &NodeType::Element(ref e) => assert_eq!(e.tag_name, "child2"),
        _ => assert!(false, "Wrong node type")
    }
}

#[test]
fn it_parse_text_node() {
    let ref node = parse("<h1>hello world</h1>".to_string())[0];
    let ref first_child = node.children[0];
    match &first_child.node_type {
        &NodeType::Text(ref c) => assert_eq!(c, "hello world"),
        _ => assert!(false, "Wrong node type")
    }
}

#[test]
fn it_parses_attributes() {
    let ref node = parse("<h1 id=\"title\">Hello world</h1>".to_string())[0];
    match &node.node_type {
        &NodeType::Element(ref e) => {
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
    let ref node = parse("<!DOCTYPE html>".to_string())[0];
    match &node.node_type {
        &NodeType::DocType(ref e) => assert_eq!(e.version, "html"),
        _ => assert!(false, "Wrong node type")
    }
}

#[test]
fn it_parses_document_as_doctype_sibling() {
    let ref node = parse("<!DOCTYPE html><html></html>".to_string())[1];
    match &node.node_type {
        &NodeType::Element(ref e) => assert_eq!(e.tag_name, "html"),
        _ => assert!(false, "Wrong node type")
    }
}
