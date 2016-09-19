use html_parser::*;
use dom::*;

#[test]
fn it_parses_element() {
    let node = get_nth_child("<test></test>".to_string(), 0);
    match node.node_type {
        NodeType::Element(e) => assert_eq!(e.tag_name, "test"),
        _ => assert!(false, "Wrong node type")
    }
}

#[test]
fn it_parses_child() {
    let node = get_nth_child("<test><child></child></test>".to_string(), 0);
    let ref first_child = node.children[0];
    match &first_child.node_type {
        &NodeType::Element(ref e) => assert_eq!(e.tag_name, "child"),
        _ => assert!(false, "Wrong node type")
    }
}

#[test]
fn it_parses_siblings() {
    let node = get_nth_child("<test><child></child><child2></child2></test>".to_string(), 0);
    let ref second_child = node.children[1];
    match &second_child.node_type {
        &NodeType::Element(ref e) => assert_eq!(e.tag_name, "child2"),
        _ => assert!(false, "Wrong node type")
    }
}

#[test]
fn it_parse_text_node() {
    let node = get_nth_child("<h1>hello world</h1>".to_string(), 0);
    let ref first_child = node.children[0];
    match &first_child.node_type {
        &NodeType::Text(ref c) => assert_eq!(c, "hello world"),
        _ => assert!(false, "Wrong node type")
    }
}

#[test]
fn it_parses_attributes() {
    let node = get_nth_child("<h1 id=\"title\">Hello world</h1>".to_string(), 0);
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
    let node = get_nth_child("<!DOCTYPE html>".to_string(), 0);
    match node.node_type {
        NodeType::DocType(ref e) => assert_eq!(e.version, "html"),
        _ => assert!(false, "Wrong node type")
    }
}

#[test]
fn it_parses_document_as_doctype_sibling() {
    let node = get_nth_child("<!DOCTYPE html><html></html>".to_string(), 1);
    match node.node_type {
        NodeType::Element(ref e) => assert_eq!(e.tag_name, "html"),
        _ => assert!(false, "Wrong node type")
    }
}

#[test]
fn it_parses_self_closing_link() {
    let node = get_nth_child("<head><link></head>".to_string(), 0);
    let ref link = node.children[0];
    match &link.node_type {
        &NodeType::Element(ref e) => assert_eq!(e.tag_name, "link"),
        _ => assert!(false, "Wrong node type")
    }
}

#[test]
fn it_parses_self_closing_meta() {
    let node = get_nth_child("<head><meta></head>".to_string(), 0);
    let ref link = node.children[0];
    match &link.node_type {
        &NodeType::Element(ref e) => assert_eq!(e.tag_name, "meta"),
        _ => assert!(false, "Wrong node type")
    }
}

#[test]
fn it_does_not_include_next_inside_self_closing() {
    let node = get_nth_child("<meta><link>".to_string(), 1);
    match node.node_type {
        NodeType::Element(e) => assert_eq!(e.tag_name, "link"),
        _ => assert!(false, "Wrong node type")
    }
}

#[test]
fn it_parses_comments() {
    let node = get_nth_child("<!-- hello world -->".to_string(), 0);
    match node.node_type {
        NodeType::Comment(c) => assert_eq!(c, " hello world "),
        _ => assert!(false, "Wrong node type")
    }
}

#[test]
fn it_parses_coments_with_dashes() {
    let node = get_nth_child("<!-- hello - world -->".to_string(), 0);
    match node.node_type {
        NodeType::Comment(c) => assert_eq!(c, " hello - world "),
        _ => assert!(false, "Wrong node type")
    }
}

#[test]
fn it_parses_cdata () {
    let node = get_nth_child("<![CDATA[<h1>Hello world]]>".to_string(), 0);
    match node.node_type {
        NodeType::CData(c) => assert_eq!(c, "<h1>Hello world"),
        _ => assert!(false, "Wrong node type")
    }
}

#[test]
fn it_parses_self_closing_tags() {
    let node = get_nth_child("<h1 /><p>Hello</p>".to_string(), 0);
    match node.node_type {
        NodeType::Element(e) => assert_eq!(e.tag_name, "h1"),
        _ => assert!(false, "Wrong node type")
    }
}

#[test]
fn child_should_close_if_parent_closed() {
    let node = get_nth_child("<ul><li>Hello</ul>".to_string(), 0);
    let ref child = node.children[0];
    match &child.node_type {
        &NodeType::Element(ref e) => assert_eq!(e.tag_name, "li"),
        _ => assert!(false, "Wrong node type")
    }
}

fn get_nth_child(text: String, pos: usize) -> Node {
    let mut warnings = vec!();
    let nodes = parse(text, &mut warnings);
    for warn in &warnings {
        println!("Warn: {}", warn)
    }
    assert_eq!(warnings.len(), 0, "No warnings expected");
    let node = nodes[pos].clone();
    return node;
}
