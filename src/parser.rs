use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct ParserPosition {
    overall: usize,
    x: usize,
    y: usize,
}

impl fmt::Display for ParserPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}  ({})", self.y, self.x, self.overall)
    }
}

pub struct Parser {
    position: ParserPosition,
    text: String,
    save_point: Option<ParserPosition>,
}

pub fn create(text: String) -> Parser {
    Parser {
        position: ParserPosition {
            overall: 0,
            x: 0,
            y: 1,
        },
        text: text,
        save_point: None,
    }
}

impl Parser {
    pub fn position(&self) -> ParserPosition {
        self.position
    }

    pub fn starts_with(&self, text: &str) -> bool {
        self.text[self.position.overall..].starts_with(text)
    }

    pub fn next_char(&self) -> char {
        self.text[self.position.overall..].chars().next().unwrap()
    }

    pub fn consume_expected_text(&mut self, text: &str) -> Result<(), String> {
        if !self.starts_with(text) {
            let value = self.consume_next_n_chars(text.len());
            Err(format!("Expected: {} Found: {}", text, value))
        } else {
            let length = text.len();
            for _ in 0..length {
                self.consume_char();
            }
            Ok(())
        }
    }

    pub fn consume_next_n_chars(&mut self, len: usize) -> String {
        let mut value = String::new();
        for _ in 0..len {
            if self.eof() {
                break;
            }
            value.push(self.consume_char());
        }
        return value;
    }

    pub fn consume_alphanumeric_word(&mut self) -> String {
        self.consume_while(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => true,
            _ => false,
        })
    }

    pub fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    pub fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        return result;
    }

    pub fn consume_until_including(&mut self, breaker: char) {
        self.consume_while(|c| c != breaker);
        self.consume_char();
    }

    pub fn consume_char(&mut self) -> char {
        let mut iter = self.text[self.position.overall..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (char_len, _) = iter.next().unwrap_or((1, ' '));
        self.position.overall += char_len;
        self.position.x += char_len;
        if cur_char == '\n' {
            self.position.y += 1;
            self.position.x = 0;
        }
        return cur_char;
    }

    pub fn eof(&self) -> bool {
        self.position.overall >= self.text.len()
    }

    pub fn set_save_point(&mut self) {
        self.save_point = Some(self.position.clone());
    }

    pub fn restore_from_save_point(&mut self) {
        self.position = self.save_point.unwrap();
        self.save_point = None;
    }
}
