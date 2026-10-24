use std::str::Chars;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Hash)]
pub struct Location<'src> {
  pub file: Option<&'src str>,
  pub byte_offset: usize,
  pub line: usize,
  pub column: usize,
}

impl<'src> Location<'src> {
  pub const fn new(file: Option<&'src str>) -> Self {
    Self {
      file,
      byte_offset: 0,
      line: 1,
      column: 0,
    }
  }

  // Only supports LF (not CRLF)!
  pub fn advance(&mut self, c: char) {
    match c {
      '\n' => {
        self.byte_offset += c.len_utf8();
        self.line += 1;
        self.column = 0;
      }
      c => {
        self.byte_offset += c.len_utf8();
        self.column += 1;
      }
    }
  }
}

impl<'src> std::fmt::Display for Location<'src> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if let Some(file) = self.file {
      f.write_str(file)?;
    }

    write!(f, "@{} {}:{}", self.byte_offset, self.line, self.column)
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Hash)]
pub struct Span<'src> {
  pub file: Option<&'src str>,
  pub start_byte_offset: usize,
  pub start_line: usize,
  pub start_column: usize,
  pub end_byte_offset: usize,
  pub end_line: usize,
  pub end_column: usize,
}

impl<'src> Span<'src> {
  pub const fn new(start: Location<'src>, end: Location<'src>) -> Self {
    Self {
      file: start.file,
      start_byte_offset: start.byte_offset,
      start_line: start.line,
      start_column: start.column,
      end_byte_offset: end.byte_offset,
      end_line: end.line,
      end_column: end.column,
    }
  }

  pub fn slice(&self, text: &'src str) -> &'src str {
    &text[self.start_byte_offset..self.end_byte_offset]
  }

  /// In bytes.
  pub fn len(&self) -> usize {
    self.end_byte_offset - self.start_byte_offset
  }
}

impl<'src> std::fmt::Display for Span<'src> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if let Some(file) = self.file {
      f.write_str(file)?;
    }

    write!(
      f,
      "@({} {}:{} - {} {}:{})",
      self.start_byte_offset,
      self.start_line,
      self.start_column,
      self.end_byte_offset,
      self.end_line,
      self.end_column,
    )
  }
}

/// An iterator. Input to parse functions.
#[derive(Debug, Clone)]
pub struct ParseSrc<'src> {
  pub src: &'src str,
  pub location: Location<'src>,
  pub chars: Chars<'src>,
}

// Handles CRLF by effectively converting to LF.
impl<'src> ParseSrc<'src> {
  pub fn new(file: Option<&'src str>, text: &'src str) -> Self {
    Self {
      src: text,
      location: Location::new(file),
      chars: text.chars(),
    }
  }

  pub fn is_empty(&self) -> bool {
    self.location.byte_offset == self.src.len()
  }

  pub fn span_from(&self, start: &ParseSrc<'src>) -> Span<'src> {
    Span::new(start.location, self.location)
  }

  // If you're wondering whether you should call this on ParseSrc A or ParseSrc B,
  // chances are it doesn't matter.
  pub fn slice(&self, span: Span<'src>) -> &'src str {
    span.slice(self.src)
  }

  pub fn zip(self, b: ParseSrc<'src>) -> ParseSrcZip<'src> {
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

impl<'src> Iterator for ParseSrc<'src> {
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

impl<'src> From<&'src str> for ParseSrc<'src> {
  fn from(value: &'src str) -> Self {
    ParseSrc::new(None, value)
  }
}

/// Like std Zip but this one allows obtaining the iterators back. An equal amount of
/// items from both sources will be consumed always.
#[derive(Debug, Clone)]
pub struct ParseSrcZip<'src> {
  pub a: ParseSrc<'src>,
  pub b: ParseSrc<'src>,
}

impl<'src> ParseSrcZip<'src> {
  pub fn new(a: ParseSrc<'src>, b: ParseSrc<'src>) -> Self {
    Self { a, b }
  }

  pub fn get(self) -> (ParseSrc<'src>, ParseSrc<'src>) {
    (self.a, self.b)
  }
}

impl<'src> Iterator for ParseSrcZip<'src> {
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
