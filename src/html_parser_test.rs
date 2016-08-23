use html_parser::*;
use dom;

#[test]
fn it_works() {
    match parse("test".to_string()) {
        Ok(node) => {
            match (node.node_type) {
                dom::NodeType::Text(c) => assert_eq!("Hello world!", c),
                dom::NodeType::Element(_) => assert!(false)
            }
        }
        Err(err) => assert_eq!("", err)
    }
}
