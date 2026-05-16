#![cfg_attr(test, feature(formatting_options))]

use std::{
  fmt::Write,
  hash::{Hash as _, Hasher},
};

pub struct HashWriter<'a, H: Hasher> {
  pub hasher: &'a mut H,
}

impl<'a, H: Hasher> Write for HashWriter<'a, H> {
  fn write_str(&mut self, s: &str) -> std::fmt::Result {
    s.hash(self.hasher);
    Ok(())
  }
}

#[cfg(test)]
mod tests {

  use std::{
    fmt::{Display, Formatter, FormattingOptions},
    hash::{DefaultHasher, Hash, Hasher as _},
  };

  use super::HashWriter;

  #[derive(Debug, Clone, Copy)]
  struct A1 {
    x: u64,
    y: u64,
  }

  impl Display for A1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{} {}", self.x, self.y)
    }
  }

  #[derive(Debug, Clone, Copy)]
  struct A2 {
    x: u64,
    y: u64,
  }

  impl Display for A2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{} {}", self.x * 2, self.y * 2)
    }
  }

  impl Hash for A1 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
      let mut writer = HashWriter { hasher: state };
      let mut fmtter = Formatter::new(&mut writer, FormattingOptions::new());
      let _ = self.fmt(&mut fmtter);
    }
  }

  impl Hash for A2 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
      let mut writer = HashWriter { hasher: state };
      let mut fmtter = Formatter::new(&mut writer, FormattingOptions::new());
      let _ = self.fmt(&mut fmtter);
    }
  }

  #[test]
  fn test() {
    let a = A1 { x: 64, y: 36 };
    let b = A2 { x: 32, y: 18 };
    let c = A2 { x: 32, y: 20 };

    let hash_a = {
      let mut hasher = DefaultHasher::new();
      a.hash(&mut hasher);
      hasher.finish()
    };

    let hash_b = {
      let mut hasher = DefaultHasher::new();
      b.hash(&mut hasher);
      hasher.finish()
    };

    let hash_c = {
      let mut hasher = DefaultHasher::new();
      c.hash(&mut hasher);
      hasher.finish()
    };

    assert_eq!(hash_a, hash_b);
    assert_ne!(hash_a, hash_c);
  }
}
