use dom;

pub fn parse(html: String) -> Vec<dom::Node> {
    let mut parser = Parser { pos: 0, input: html, line_num: 1 };
    parser.parse_nodes()
}

struct Parser {
    pos: usize,
    input: String,
    line_num: usize
}

impl Parser {
    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = vec!();
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            match self.parse_node() {
                Ok(node) => nodes.push(node),
                Err(err) => println!("Line: {}, {}", self.line_num, err)
            }
        }
        return nodes;
    }

    fn parse_node(&mut self) -> Result<dom::Node, String> {
        match self.next_char() {
            '<' => {
                if self.starts_with("<!--") {
                    self.parse_comment()
                } else if self.starts_with("<!") {
                    self.parse_doctype()
                } else {
                    self.parse_element()
                }
            },
            _ => self.parse_text()
        }
    }

    fn parse_text(&mut self) -> Result<dom::Node, String> {
        let text = self.consume_while(|c| match c {
            '<' => false,
            _ => true
        });
        Ok(dom::text(text))
    }

    fn parse_doctype(&mut self) -> Result<dom::Node, String> {
        match self.consume_expected_text("<!DOCTYPE") {
            Some(e) => return Err(e),
            None => {}
        }
        self.consume_whitespace();
        let version = self.consume_alphanumeric_word();
        self.consume_whitespace();
        match self.consume_expected_text(">") {
            Some(e) => return Err(e),
            None => {}
        }
        Ok(dom::doctype(version))
    }

    fn parse_comment(&mut self) -> Result<dom::Node, String> {
        self.consume_expected_text("<!--");
        let mut comment = String::new();
        loop {
            let partial = self.consume_while(|c| match c {
                '-' => false,
                _ => true
            });
            comment.push_str(partial.as_str());
            if self.eof() || self.starts_with("-->") {
                break;
            } else {
                comment.push(self.consume_char());
            }
        }
        self.consume_expected_text("-->");
        Ok(dom::comment(comment))
    }

    fn parse_element(&mut self) -> Result<dom::Node, String> {
        match self.consume_expected_text("<") {
            Some(e) => return Err(e),
            _ => {}
        }
        let (tag_name, attributes) = match self.parse_tag() {
            Ok((tag_name, attributes)) => (tag_name, attributes),
            Err(e) => return Err(e)
        };
        let children = self.parse_nodes();
        self.consume_closing_tag(tag_name.as_str());
        self.consume_whitespace();
        Ok(dom::element(tag_name, attributes, children))
    }

    fn parse_tag(&mut self) -> Result<(String, dom::AttrMap), String> {
        let tag_name = self.parse_tag_name();
        let attributes = match self.parse_attributes() {
            Ok(atts) => atts,
            Err(e) => return Err(e)
        };

        self.consume_whitespace();
        let ending_len = if self.starts_with("/") { 2 } else { 1 };
        let ending = self.consume_next_n_chars(ending_len);
        if ending.ends_with(">") {
            Ok((tag_name, attributes))
        } else {
            Err(format!("Expected end of tag but found: {}", ending))
        }
    }

    fn parse_tag_name(&mut self) -> String {
        self.consume_whitespace();
        self.consume_alphanumeric_word()
    }

    fn parse_attributes(&mut self) ->  Result<dom::AttrMap, String> {
        let mut attrs = dom::AttrMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' || self.next_char() == '/' {
                break;
            }
            let name = self.parse_attribute_name();
            let next_char = self.consume_char();
            if next_char != '=' {
                return Err(format!("Unexpected char in attribute parsing Expected: = found: {}", next_char));
            }
            let value = match self.parse_attribute_value() {
                Ok(v) => v,
                Err(e) => return Err(e)
            };
            attrs.insert(name, value);
        }
        Ok(attrs)
    }

    fn parse_attribute_name(&mut self) -> String {
        self.consume_while(|c| match c {
            'a'...'z' | 'A'...'Z' | '0'...'9' | '-' | '_' => true,
            _ => false
        })
    }

    fn parse_attribute_value(&mut self) -> Result<String, String> {
        let first_char = self.consume_char();
        if first_char != '"' && first_char != '\'' {
            return Err(format!("Expected opening of attribute value but found: {}", first_char));
        }
        let value = self.consume_while(|c| match c {
            '"' | '\'' => false,
            _ => true
        });
        self.consume_char();
        Ok(value)
    }

    fn consume_closing_tag(&mut self, tag_name: &str) -> Option<String> {
        if tag_name == "link" || tag_name == "meta" {
            None
        } else {
            let closing_tag = "</".to_owned() + tag_name + ">";
            self.consume_expected_text(closing_tag.as_str())
        }
    }

    fn starts_with(&self, text: &str) -> bool {
        self.input[self.pos..].starts_with(text)
    }

    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn consume_expected_text(&mut self, text: &str) -> Option<String> {
        if !self.starts_with(text) {
            let value = self.consume_next_n_chars(text.len());
            Some(format!("Expected: {} Found: {}", text, value))
        } else {
            let length = text.len();
            for _ in 0..length {
                self.consume_char();
            }
            None
        }
    }

    fn consume_next_n_chars(&mut self, len: usize) -> String {
        let mut value = String::new();
        for _ in 0..len {
            if self.eof() {
                break;
            }
            value.push(self.consume_char());
        }
        return value;
    }

    fn consume_alphanumeric_word(&mut self) -> String {
        self.consume_while(|c| match c {
            'a'...'z' | 'A'...'Z' | '0'...'9' => true,
            _ => false
        })
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    fn consume_while<F>(&mut self, test: F) -> String
        where F: Fn(char) -> bool {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        return result;
    }

    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (char_len, _) = iter.next().unwrap_or((1, ' '));
        self.pos += char_len;
        if cur_char == '\n' {
            self.line_num += 1;
        }
        return cur_char;
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }
}
