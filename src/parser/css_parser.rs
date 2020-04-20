use parser::parser;

#[derive(Clone)]
pub struct StyleSheet {
    pub rules: Vec<Rule>,
}

#[derive(Clone)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

#[derive(Clone)]
pub enum Selector {
    Simple(SimpleSelector),
}

#[derive(Clone)]
pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub class: Vec<String>,
}

#[derive(Clone)]
pub struct Declaration {
    pub name: String,
    pub value: Value,
}

#[derive(Clone)]
pub enum Value {
    // Keyword(String),
    Length(f32, Unit),
    // ColorValue(Color),
}

#[derive(Clone, Copy)]
pub enum Unit {
    Px,
}

// #[derive(Clone, Copy)]
// pub struct Color {
//     r: u8,
//     g: u8,
//     b: u8,
//     a: u8,
// }

pub fn parse<'a>(style_text: String, warnings: &'a mut Vec<String>) -> StyleSheet {
    let mut parser = CSSParser {
        parser: parser::create(style_text),
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
            selectors.push(Selector::Simple(self.consume_simple_selector()));
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

    fn consume_simple_selector(&mut self) -> SimpleSelector {
        let mut selector = SimpleSelector {
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
        match self.consume_value() {
            Ok(value) => {
                self.parser.consume_whitespace();
                self.parser.consume_expected_text(";").unwrap();
                Ok(Declaration { name, value })
            }
            Err(e) => Err(e),
        }
    }

    fn consume_value(&mut self) -> Result<Value, String> {
        match self.parser.next_char() {
            '0'..='9' => self.consume_length(),
            _ => Err(format!(
                "ERROR@{} - Consuming value - Invalid character: '{}'",
                self.parser.position(),
                self.parser.next_char()
            )),
        }
    }

    fn consume_length(&mut self) -> Result<Value, String> {
        let amount = self.consume_float();

        match self.consume_unit() {
            Ok(unit) => Ok(Value::Length(amount, unit)),
            Err(e) => Err(e),
        }
    }

    fn consume_float(&mut self) -> f32 {
        let text = self.parser.consume_while(|c| match c {
            '0'..='9' | '.' => true,
            _ => false,
        });
        text.parse().unwrap()
    }

    fn consume_unit(&mut self) -> Result<Unit, String> {
        match self.parser.next_char().to_ascii_lowercase() {
            'p' => {
                self.parser.consume_expected_text("px").unwrap();
                Ok(Unit::Px)
            }
            _ => Err(format!(
                "ERROR@{} - Consuming unit found: '{}'",
                self.parser.position(),
                self.parser.next_char()
            )),
        }
    }

    fn consume_identifier(&mut self) -> String {
        self.parser.consume_while(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => true,
            _ => false,
        })
    }
}
