use std::{borrow::Cow, fmt::Write as _};

use crate::parse::data::str::Span;

#[derive(Debug, Clone)]
pub struct ResolveError<'a> {
  span: Span<'a>,
  message: Cow<'static, str>,
}

impl<'a> std::fmt::Display for ResolveError<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.span.fmt(f)?;
    f.write_char(':')?;
    f.write_str(self.message.as_ref())?;
    Ok(())
  }
}

impl<'a> ResolveError<'a> {
  pub fn new<T>(span: Span<'a>, message: T) -> Self
  where
    T: Into<Cow<'static, str>>,
  {
    Self {
      span,
      message: message.into(),
    }
  }

  // pub fn wrap_message<T>(self, msg: T) -> ResolveError<'a>
  // where
  //   T: Into<Cow<'static, str>>,
  // {
  //   let wrapped_msg = format!("{}: {}", msg.into(), self.message.as_ref());
  //   self.sub_message(Cow::Owned(wrapped_msg))
  // }

  // pub fn sub_message<T>(self, msg: T) -> ResolveError<'a>
  // where
  //   T: Into<Cow<'static, str>>,
  // {
  //   ResolveError {
  //     span: self.span,
  //     message: msg.into(),
  //   }
  // }
}

pub type ResolveResult<'a, T> = Result<T, ResolveError<'a>>;
