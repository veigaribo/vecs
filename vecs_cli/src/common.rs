use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringKind {
  DoubleQuoted(String),
  AngleBracketed(String),
}

impl Display for StringKind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      StringKind::DoubleQuoted(x) => write!(f, "\"{}\"", x),
      StringKind::AngleBracketed(x) => write!(f, "<{}>", x),
    }
  }
}
