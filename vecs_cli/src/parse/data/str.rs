#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Location<'str> {
  pub file: Option<&'str str>,
  pub byte_offset: usize,
  pub line: usize,
  pub column: usize,
}

impl<'str> Location<'str> {
  pub const fn new(file: Option<&'str str>) -> Self {
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

impl<'str> std::fmt::Display for Location<'str> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if let Some(file) = self.file {
      f.write_str(file)?;
    }

    write!(f, "@{} {}:{}", self.byte_offset, self.line, self.column)
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Span<'str> {
  pub file: Option<&'str str>,
  pub start_byte_offset: usize,
  pub start_line: usize,
  pub start_column: usize,
  pub end_byte_offset: usize,
  pub end_line: usize,
  pub end_column: usize,
}

impl<'str> Span<'str> {
  pub const fn new(start: Location<'str>, end: Location<'str>) -> Self {
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

  pub fn slice(&self, text: &'str str) -> &'str str {
    &text[self.start_byte_offset..self.end_byte_offset]
  }

  /// In bytes.
  pub fn len(&self) -> usize {
    self.end_byte_offset - self.start_byte_offset
  }
}

impl<'str> std::fmt::Display for Span<'str> {
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
