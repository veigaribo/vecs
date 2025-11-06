use std::{borrow::Cow, fmt::Write as _};

use crate::parse::data::str::Span;

#[derive(Debug, Clone)]
pub struct ResolveError<'src> {
  span: Span<'src>,
  message: Cow<'static, str>,
}

impl<'src> std::fmt::Display for ResolveError<'src> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.span.fmt(f)?;
    f.write_char(':')?;
    f.write_str(self.message.as_ref())?;
    Ok(())
  }
}

impl<'src> ResolveError<'src> {
  pub fn new<T>(span: Span<'src>, message: T) -> Self
  where
    T: Into<Cow<'static, str>>,
  {
    Self {
      span,
      message: message.into(),
    }
  }

  // pub fn wrap_message<T>(self, msg: T) -> ResolveError<'src>
  // where
  //   T: Into<Cow<'static, str>>,
  // {
  //   let wrapped_msg = format!("{}: {}", msg.into(), self.message.as_ref());
  //   self.sub_message(Cow::Owned(wrapped_msg))
  // }

  // pub fn sub_message<T>(self, msg: T) -> ResolveError<'src>
  // where
  //   T: Into<Cow<'static, str>>,
  // {
  //   ResolveError {
  //     span: self.span,
  //     message: msg.into(),
  //   }
  // }
}

pub type ResolveResult<'src, T> = Result<T, ResolveError<'src>>;
