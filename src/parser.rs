pub struct Parser {
  pub position: usize,
  pub line_number: usize,
  text: String,
}

pub fn create(text: String) -> Parser {
  Parser {
    position: 0,
    line_number: 1,
    text: text,
  }
}

impl Parser {
  pub fn starts_with(&self, text: &str) -> bool {
    self.text[self.position..].starts_with(text)
  }

  pub fn next_char(&self) -> char {
    self.text[self.position..].chars().next().unwrap()
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

  pub fn consume_char(&mut self) -> char {
    let mut iter = self.text[self.position..].char_indices();
    let (_, cur_char) = iter.next().unwrap();
    let (char_len, _) = iter.next().unwrap_or((1, ' '));
    self.position += char_len;
    if cur_char == '\n' {
      self.line_number += 1;
    }
    return cur_char;
  }

  pub fn eof(&self) -> bool {
    self.position >= self.text.len()
  }
}
