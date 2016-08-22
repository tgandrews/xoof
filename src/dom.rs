use std::collections::{HashMap,HashSet};
use std::fmt;

#[derive(Debug)]
enum NodeType {
    Text(String),
    Element(ElementData),
}

#[derive(Debug)]
pub struct Node {
    pub children: Vec<Node>,
    node_type: NodeType,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.print(0))
    }
}

impl Node {
    fn print(&self, depth: usize) -> String {
        let tag = match self.node_type {
            NodeType::Text(_) => "Text".to_string(),
            NodeType::Element(ref elem) => elem.tag_name.clone()
        };
        let mut indent = String::new();
        for _ in 0..depth {
            indent.push_str("  ");
        }
        let mut children_output = String::new();
        let next_depth = depth + 1;
        for child in &self.children {
            children_output += child.print(next_depth).as_str();
            children_output += "\n";
        }
        return indent + tag.as_str() + "\n" + children_output.as_str();
    }
}

#[derive(Debug)]
struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

pub type AttrMap = HashMap<String, String>;


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
