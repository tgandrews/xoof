use dom;

pub fn parse(html: String) -> Result<dom::Node, String> {
    let mut parser = Parser { pos: 0, input: html };
    parser.parse_node()
}

struct Parser {
    pos: usize,
    input: String
}

impl Parser {
    fn parse_nodes(&mut self) -> Result<Vec<dom::Node>, String> {
        let mut nodes = vec!();
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            match self.parse_node() {
                Ok(node) => nodes.push(node),
                Err(e) => return Err(e)
            }
        }
        return Ok(nodes);
    }

    fn parse_node(&mut self) -> Result<dom::Node, String> {
        match self.next_char() {
            '<' => self.parse_element(),
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

    fn parse_element(&mut self) -> Result<dom::Node, String> {
        if self.consume_char() != '<' {
            return Err("Expected opening tag".to_string());
        }
        let (tag_name, attributes) = self.parse_tag();

        let children = match self.parse_nodes() {
            Ok(c) => c,
            Err(e) => return Err(e)
        };
        self.consume_closing_tag(tag_name.as_str());
        self.consume_whitespace();
        Ok(dom::element(tag_name, attributes, children))
    }

    fn parse_tag(&mut self) -> (String, dom::AttrMap) {
        let tag_name = self.parse_tag_name();
        let attributes = self.parse_attributes();
        self.consume_whitespace();
        assert_eq!('>', self.consume_char());
        return (tag_name, attributes);
    }

    fn parse_tag_name(&mut self) -> String {
        self.consume_whitespace();
        self.consume_alphanumeric_word()
    }

    fn parse_attributes(&mut self) -> dom::AttrMap {
        let mut attrs = dom::AttrMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let name = self.parse_attribute_name();
            assert_eq!('=', self.consume_char());
            let value = self.parse_attribute_value();
            attrs.insert(name, value);

        }
        return attrs;
    }

    fn parse_attribute_name(&mut self) -> String {
        self.consume_alphanumeric_word()
    }

    fn parse_attribute_value(&mut self) -> String {
        assert_eq!('"', self.consume_char());
        let value = self.consume_while(|c| match c {
            '"' => false,
            _ => true
        });
        assert_eq!('"', self.consume_char());
        return value;
    }

    fn consume_closing_tag(&mut self, tag_name: &str) {
        let closing_tag = "</".to_owned() + tag_name + ">";
        if !self.consume_expected_text(closing_tag.as_str()) {
            return assert!(false, "Expected closing tag for: ".to_owned() + tag_name)
        }
    }

    fn starts_with(&self, text: &str) -> bool {
        self.input[self.pos..].starts_with(text)
    }

    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn consume_expected_text(&mut self, text: &str) -> bool {
        if !self.starts_with(text) {
            false
        } else {
            let length = text.len();
            for _ in 0..length {
                self.consume_char();
            }
            true
        }
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
        return cur_char;
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }
}
