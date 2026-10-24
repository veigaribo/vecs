use std::fmt::Display;

// IMPORTANT: Keep in sync with `/vecs/c/lib.h`!
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum StringKind {
  DoubleQuoted(String) = 1,
  AngleBracketed(String) = 2,
}

// https://doc.rust-lang.org/core/mem/fn.discriminant.html#accessing-the-numeric-value-of-the-discriminant
impl StringKind {
  pub fn discriminant(&self) -> u8 {
    unsafe { *<*const _>::from(self).cast::<u8>() }
  }
}

impl Display for StringKind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      StringKind::DoubleQuoted(x) => write!(f, "\"{}\"", x),
      StringKind::AngleBracketed(x) => write!(f, "<{}>", x),
    }
  }
}
