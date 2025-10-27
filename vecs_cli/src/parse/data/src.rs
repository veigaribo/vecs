use std::str::Chars;

use crate::parse::data::str::Span;

use super::str::Location;

/// An iterator. Input to parse functions.
#[derive(Debug, Clone)]
pub struct ParseSrc<'str> {
  pub src: &'str str,
  pub location: Location<'str>,
  pub chars: Chars<'str>,
}

// Handles CRLF by effectively converting to LF.
impl<'str> ParseSrc<'str> {
  pub fn new(file: Option<&'str str>, text: &'str str) -> Self {
    Self {
      src: text,
      location: Location::new(file),
      chars: text.chars(),
    }
  }

  pub fn is_empty(&self) -> bool {
    self.location.byte_offset == self.src.len()
  }

  pub fn span_from(&self, start: &ParseSrc<'str>) -> Span<'str> {
    Span::new(start.location, self.location)
  }

  // If you're wondering whether you should call this on ParseSrc A or ParseSrc B,
  // chances are it doesn't matter.
  pub fn slice(&self, span: Span<'str>) -> &'str str {
    span.slice(self.src)
  }

  pub fn zip(self, b: ParseSrc<'str>) -> ParseSrcZip<'str> {
    ParseSrcZip::new(self, b)
  }

  pub fn peek(&self) -> Option<char> {
    let mut clone = self.chars.clone();
    clone.next()
  }

  pub fn advance_bytes(&mut self, bytes: usize) {
    let start_offset = self.location.byte_offset;
    let final_offset = start_offset + bytes;

    while self.location.byte_offset < final_offset {
      self.next();
    }
  }

  // We don't really have the concept of a token, but this is useful to make error
  // messages.
  pub fn peek_next_token_ish(&self) -> String {
    let max_len = 100;
    let clone = self.clone();

    let mut accum = String::new();

    for c in clone {
      if c.is_whitespace() {
        break;
      }

      if accum.len() >= max_len {
        accum.push_str("...");
        break;
      }

      accum.push(c);
    }

    accum
  }

  #[cfg(test)]
  pub fn remaining_str(&self) -> String {
    self.chars.clone().collect::<String>()
  }
}

impl<'str> Iterator for ParseSrc<'str> {
  type Item = char;

  fn next(&mut self) -> Option<Self::Item> {
    let next = self.chars.next();

    if let Some(next_char) = next {
      self.location.advance(next_char);

      // Handle CRLF right here and now.
      let next_char = {
        if next_char == '\r' {
          let chars_backup = self.chars.clone();
          let next_next_char = self.chars.next();

          if let Some('\n') = next_next_char {
            self.location.advance(next_char);
            '\n'
          } else {
            self.chars = chars_backup;
            next_char
          }
        } else {
          next_char
        }
      };

      Some(next_char)
    } else {
      None
    }
  }
}

impl<'str> From<&'str str> for ParseSrc<'str> {
  fn from(value: &'str str) -> Self {
    ParseSrc::new(None, value)
  }
}

/// Like std Zip but this one allows obtaining the iterators back. An equal amount of
/// items from both sources will be consumed always.
#[derive(Debug, Clone)]
pub struct ParseSrcZip<'str> {
  pub a: ParseSrc<'str>,
  pub b: ParseSrc<'str>,
}

impl<'str> ParseSrcZip<'str> {
  pub fn new(a: ParseSrc<'str>, b: ParseSrc<'str>) -> Self {
    Self { a, b }
  }

  pub fn get(self) -> (ParseSrc<'str>, ParseSrc<'str>) {
    (self.a, self.b)
  }
}

impl<'str> Iterator for ParseSrcZip<'str> {
  type Item = (char, char);

  fn next(&mut self) -> Option<Self::Item> {
    let a_start = self.a.clone();
    let b_start = self.b.clone();

    match self.a.next() {
      Some(a_item) => match self.b.next() {
        Some(b_item) => Some((a_item, b_item)),
        None => {
          self.a = a_start;
          self.b = b_start;
          None
        }
      },
      None => {
        self.a = a_start;
        None
      }
    }
  }
}
