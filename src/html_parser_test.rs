use html_parser::*;
use dom;

#[test]
fn it_works() {
    match parse("<test></test>".to_string()) {
        Ok(node) => {
            match node.node_type {
                dom::NodeType::Element(e) => assert_eq!(e.tag_name, "test"),
                _ => assert!(false, "Wrong node type")
            }
        }
        _ => assert!(false)
    }
}
