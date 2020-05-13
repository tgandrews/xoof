use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
pub enum NodeType {
    Comment(String),
    CData(String),
    DocType(DocTypeData),
    Element(ElementData),
    Text(String),
}

#[derive(Debug, Clone)]
pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.pretty_print(0))
    }
}

impl Node {
    fn pretty_print(&self, depth: usize) -> String {
        let tag = match self.node_type {
            NodeType::Text(ref content) => String::from("Text {") + content.as_str() + "}",
            NodeType::DocType(ref elem) => String::from("DocType {") + elem.version.as_str() + "}",
            NodeType::Comment(ref comment) => String::from("Comment {") + comment.as_str() + "}",
            NodeType::CData(ref content) => String::from("CData {") + content.as_str() + "}",
            NodeType::Element(ref elem) => {
                let mut output = elem.tag_name.clone();
                output.push_str(" {");
                let mut first = true;
                for (key, value) in &elem.attributes {
                    if !first {
                        output.push_str(", ");
                    }
                    output.push_str(key);
                    output.push_str(": \"");
                    output.push_str(value);
                    output.push_str("\"");
                    first = false;
                }
                output.push_str("}");
                output
            }
        };
        let indent = (0..depth)
            .map(|_| String::from(" "))
            .collect::<Vec<String>>()
            .join("");
        let mut children_output = String::new();
        let next_depth = depth + 1;
        for child in &self.children {
            children_output += child.pretty_print(next_depth).as_str();
        }
        return indent + tag.as_str() + "\n" + children_output.as_str();
    }
}

#[derive(Debug, Clone)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: AttrMap,
}

#[derive(Debug, Clone)]
pub struct DocTypeData {
    pub version: String,
}

pub type AttrMap = HashMap<String, String>;

pub fn doctype(version: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::DocType(DocTypeData { version: version }),
    }
}

pub fn text(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Text(data),
    }
}

pub fn comment(comment: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Comment(comment),
    }
}

pub fn cdata(content: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::CData(content),
    }
}

pub fn element(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        }),
    }
}
