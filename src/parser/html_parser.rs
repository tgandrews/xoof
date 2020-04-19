use dom;
use parser;

pub fn parse(html: String, warnings: &mut Vec<String>) -> Vec<dom::Node> {
    let mut parser = HTMLParser {
        parser: parser::parser::create(html),
        stack: vec![],
    };
    parser.parse_nodes(warnings)
}

struct HTMLParser {
    parser: parser::parser::Parser,
    stack: Vec<String>,
}

impl HTMLParser {
    fn parse_nodes(&mut self, warnings: &mut Vec<String>) -> Vec<dom::Node> {
        let mut nodes = vec![];
        loop {
            self.parser.consume_whitespace();
            if self.parser.eof() || self.parser.starts_with("</") {
                break;
            }
            match self.parse_node(warnings) {
                Ok(node) => nodes.push(node),
                Err(err) => warnings.push(format!("{} - {}", self.parser.position(), err)),
            }
        }
        return nodes;
    }

    fn parse_node(&mut self, warnings: &mut Vec<String>) -> Result<dom::Node, String> {
        match self.parser.next_char() {
            '<' => {
                if self.parser.starts_with("<!--") {
                    self.parse_comment()
                } else if self.parser.starts_with("<![CDATA[") {
                    self.parse_cdata()
                } else if self.parser.starts_with("<!") {
                    self.parse_doctype()
                } else {
                    self.parse_element(warnings)
                }
            }
            _ => self.parse_text(),
        }
    }

    fn parse_text(&mut self) -> Result<dom::Node, String> {
        let text = self.parser.consume_while(|c| match c {
            '<' => false,
            _ => true,
        });
        Ok(dom::text(text))
    }

    fn parse_doctype(&mut self) -> Result<dom::Node, String> {
        match self.parser.consume_expected_text("<!DOCTYPE") {
            Err(e) => return Err(e),
            _ => {}
        }
        self.parser.consume_whitespace();
        let version = self.parser.consume_alphanumeric_word();
        self.parser.consume_whitespace();
        match self.parser.consume_expected_text(">") {
            Err(e) => return Err(e),
            _ => {}
        }
        Ok(dom::doctype(version))
    }

    fn parse_comment(&mut self) -> Result<dom::Node, String> {
        match self.parser.consume_expected_text("<!--") {
            Err(e) => return Err(e),
            _ => {}
        };
        let mut comment = String::new();
        loop {
            let partial = self.parser.consume_while(|c| match c {
                '-' => false,
                _ => true,
            });
            comment.push_str(partial.as_str());
            if self.parser.eof() || self.parser.starts_with("-->") {
                break;
            } else {
                comment.push(self.parser.consume_char());
            }
        }
        match self.parser.consume_expected_text("-->") {
            Err(e) => return Err(e),
            _ => {}
        };
        Ok(dom::comment(comment))
    }

    fn parse_cdata(&mut self) -> Result<dom::Node, String> {
        match self.parser.consume_expected_text("<![CDATA[") {
            Err(e) => return Err(e),
            _ => {}
        }
        let mut comment = String::new();
        loop {
            let partial = self.parser.consume_while(|c| match c {
                ']' => false,
                _ => true,
            });
            comment.push_str(partial.as_str());
            if self.parser.eof() || self.parser.starts_with("]]>") {
                break;
            } else {
                comment.push(self.parser.consume_char());
            }
        }
        match self.parser.consume_expected_text("]]>") {
            Err(e) => return Err(e),
            _ => {}
        }
        Ok(dom::cdata(comment))
    }

    fn parse_element(&mut self, warnings: &mut Vec<String>) -> Result<dom::Node, String> {
        match self.parser.consume_expected_text("<") {
            Err(e) => return Err(e),
            _ => {}
        }
        let (tag_name, attributes, has_closed_self) = match self.parse_tag() {
            Ok((tag_name, attributes, self_closed)) => (tag_name, attributes, self_closed),
            Err(e) => return Err(e),
        };
        let closed = has_closed_self || self.is_self_closing(tag_name.as_str());
        let mut children = vec![];
        if !closed {
            self.stack.push(tag_name.clone());
            children = self.parse_nodes(warnings);
            self.stack.pop();
            match self.consume_closing_tag(tag_name.as_str()) {
                Err(e) => return Err(e),
                _ => {}
            }
        }
        Ok(dom::element(tag_name, attributes, children))
    }

    fn parse_tag(&mut self) -> Result<(String, dom::AttrMap, bool), String> {
        let tag_name = self.parse_tag_name();
        let attributes = match self.parse_attributes() {
            Ok(atts) => atts,
            Err(e) => return Err(e),
        };

        self.parser.consume_whitespace();
        let has_closed_self = self.parser.starts_with("/");
        let ending_len = if has_closed_self { 2 } else { 1 };
        let ending = self.parser.consume_next_n_chars(ending_len);
        if ending.ends_with(">") {
            Ok((tag_name, attributes, has_closed_self))
        } else {
            Err(format!("Expected end of tag but found: {}", ending))
        }
    }

    fn parse_tag_name(&mut self) -> String {
        self.parser.consume_whitespace();
        self.parser.consume_alphanumeric_word()
    }

    fn parse_attributes(&mut self) -> Result<dom::AttrMap, String> {
        let mut attrs = dom::AttrMap::new();
        loop {
            self.parser.consume_whitespace();
            if self.parser.next_char() == '>' || self.parser.next_char() == '/' {
                break;
            }
            let name = self.parse_attribute_name();
            let next_char = self.parser.consume_char();
            if next_char != '=' {
                return Err(format!(
                    "Unexpected char in attribute parsing Expected: = found: {}",
                    next_char
                ));
            }
            let value = match self.parse_attribute_value() {
                Ok(v) => v,
                Err(e) => return Err(e),
            };
            attrs.insert(name, value);
        }
        Ok(attrs)
    }

    fn parse_attribute_name(&mut self) -> String {
        self.parser.consume_while(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => true,
            _ => false,
        })
    }

    fn parse_attribute_value(&mut self) -> Result<String, String> {
        let first_char = self.parser.consume_char();
        if first_char != '"' && first_char != '\'' {
            return Err(format!(
                "Expected opening of attribute value but found: {}",
                first_char
            ));
        }

        let value = self.parser.consume_while(|c| c != first_char);
        self.parser.consume_char();
        Ok(value)
    }

    fn consume_closing_tag(&mut self, tag_name: &str) -> Result<(), String> {
        self.parser.set_save_point();
        match self.parser.consume_expected_text("</") {
            Err(e) => return Err(e),
            _ => {}
        }
        let closing_tag_name = self.parse_tag_name();
        self.parser.consume_whitespace();
        match self.parser.consume_expected_text(">") {
            Err(e) => return Err(e),
            _ => {}
        }
        if closing_tag_name == tag_name {
            return Ok(());
        }
        for parent_tag_name in &self.stack {
            if &closing_tag_name == parent_tag_name {
                self.parser.restore_from_save_point();
                return Ok(());
            }
        }
        Err(format!("Expected closing tag for: {} but found closing tag for: {} which is not in the stack: {:?}",
                tag_name, closing_tag_name, self.stack))
    }

    fn is_self_closing(&self, tag_name: &str) -> bool {
        match tag_name {
            "link" | "meta" => true,
            _ => false,
        }
    }
}
