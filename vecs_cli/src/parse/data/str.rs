use std::fmt::Write as _;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

impl<'str> std::fmt::Display for Location<'str> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if let Some(file) = self.file {
      f.write_str(file)?;
    }

    f.write_char('@')?;
    self.byte_offset.fmt(f)?;
    f.write_char(' ')?;
    self.line.fmt(f)?;
    self.column.fmt(f)?;

    Ok(())
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

  /// In bytes.
  pub fn len(&self) -> usize {
    self.end.byte_offset - self.start.byte_offset
  }
}
