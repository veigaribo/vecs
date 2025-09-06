#[derive(Debug, Copy, Clone)]
pub struct Location<'str> {
  pub file: Option<&'str str>,
  pub byte_offset: usize,
  pub line: usize,
  pub column: usize,
}

impl<'str> Location<'str> {
  pub fn new(file: Option<&'str str>) -> Self {
    Self {
      file,
      byte_offset: 0,
      line: 0,
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

#[derive(Debug, Copy, Clone)]
pub struct Span<'str> {
  pub start: Location<'str>,
  pub end: Location<'str>,
}

impl<'str> Span<'str> {
  pub fn new(start: Location<'str>, end: Location<'str>) -> Self {
    Self { start, end }
  }

  pub fn slice(&self, text: &'str str) -> &'str str {
    &text[self.start.byte_offset..self.end.byte_offset]
  }

  pub fn len(&self) -> usize {
    self.end.byte_offset - self.start.byte_offset
  }
}
