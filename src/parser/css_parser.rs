use cssom::*;
use parser::parser;

pub fn parse<'a>(css_text: String, warnings: &'a mut Vec<String>) -> StyleSheet {
    let mut parser = CSSParser {
        parser: parser::create(css_text),
        warnings,
    };
    return StyleSheet {
        rules: parser.parse_rules(),
    };
}

struct CSSParser<'a> {
    parser: parser::Parser,
    warnings: &'a mut Vec<String>,
}

impl<'a> CSSParser<'a> {
    fn parse_rules(&mut self) -> Vec<Rule> {
        let mut rules = vec![];
        while !self.parser.eof() {
            self.parser.consume_whitespace();
            rules.push(self.consume_rule());
            self.parser.consume_whitespace();
        }
        rules
    }

    fn consume_rule(&mut self) -> Rule {
        let selectors = self.consume_selectors();
        let declarations = self.consume_declarations();
        Rule {
            selectors,
            declarations,
        }
    }

    fn consume_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = vec![];
        loop {
            self.parser.consume_whitespace();
            selectors.push(Selector {
                selector_type: SelectorType::SimpleSelector(self.consume_simple_selector()),
            });
            self.parser.consume_whitespace();
            match self.parser.next_char() {
                ',' => {
                    self.parser.consume_char();
                }
                '{' => break,
                _ => self.warnings.push(format!(
                    "Consuming selectors: @{} - {}",
                    self.parser.position(),
                    self.parser.next_char()
                )),
            }
        }

        selectors
    }

    fn consume_simple_selector(&mut self) -> SimpleSelectorData {
        let mut selector = SimpleSelectorData {
            tag_name: None,
            id: None,
            class: vec![],
        };
        match self.parser.next_char() {
            '#' => {
                self.parser.consume_char();
                selector.id = Some(self.consume_identifier());
            }
            '.' => {
                self.parser.consume_char();
                selector.class.push(self.consume_identifier());
            }
            _ => {
                selector.tag_name = Some(self.consume_identifier());
            }
        };
        selector
    }

    fn consume_declarations(&mut self) -> Vec<Declaration> {
        let mut declarations = vec![];
        self.parser.consume_expected_text("{").unwrap();
        while self.parser.next_char() != '}' {
            self.parser.consume_whitespace();
            match self.consume_declaration() {
                Ok(declaration) => {
                    declarations.push(declaration);
                }
                Err(e) => {
                    self.parser.consume_until_including(';');
                    self.warnings.push(format!("Skipping declaration - {}", e));
                }
            }
            self.parser.consume_whitespace();
        }
        self.parser.consume_expected_text("}").unwrap();
        declarations
    }

    fn consume_declaration(&mut self) -> Result<Declaration, String> {
        let name = self.consume_identifier();
        self.parser.consume_whitespace();
        self.parser.consume_expected_text(":").unwrap();
        self.parser.consume_whitespace();
        let value = self.consume_value();
        Ok(Declaration { name, value })
    }

    fn consume_value(&mut self) -> String {
        let value = self.parser.consume_while(|c| c != ';');
        self.parser.consume_expected_text(";").unwrap();
        value
    }

    fn consume_identifier(&mut self) -> String {
        self.parser.consume_while(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => true,
            _ => false,
        })
    }
}
