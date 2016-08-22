use std::collections::{HashMap};
use std::fmt;

#[derive(Debug)]
enum NodeType {
    Text(String),
    Element(ElementData),
}

#[derive(Debug)]
struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

pub type AttrMap = HashMap<String, String>;

#[derive(Debug)]
pub struct Node {
    pub children: Vec<Node>,
    node_type: NodeType,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.pretty_print(0))
    }
}

impl Node {
    fn pretty_print(&self, depth: usize) -> String {
        let tag = match self.node_type {
            NodeType::Text(ref content) => {
                "Text {".to_owned() + content.as_str() + "}"
            },
            NodeType::Element(ref elem) => {
                let mut output = elem.tag_name.clone();
                output.push_str(" {");
                let mut first = true;
                for (k, v) in &elem.attributes {
                    if !first {
                        output.push_str(", ");
                    }
                    output.push_str(k);
                    output.push_str(": \"");
                    output.push_str(v);
                    output.push_str("\"");
                    first = false;
                }
                output.push_str("}");
                output
            }
        };
        let mut indent = String::new();
        for _ in 0..depth {
            indent.push_str("  ");
        }
        let mut children_output = String::new();
        let next_depth = depth + 1;
        for child in &self.children {
            children_output += child.pretty_print(next_depth).as_str();
        }
        return indent + tag.as_str() + "\n" + children_output.as_str();
    }
}


pub fn text(data: String) -> Node {
    Node { children: Vec::new(), node_type: NodeType::Text(data) }
}

pub fn element(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children: children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        })
    }
}
