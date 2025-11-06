use std::{borrow::Cow, fmt::Write as _};

use crate::parse::data::{src::ParseSrc, str::Location};

use super::str::Span;

#[derive(Debug, Clone)]
pub struct ParseError<'src> {
  location: Location<'src>,
  message: Cow<'static, str>,
}

impl<'src> std::fmt::Display for ParseError<'src> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.location.fmt(f)?;
    f.write_char(':')?;
    f.write_str(self.message.as_ref())?;
    Ok(())
  }
}

impl<'src> ParseError<'src> {
  pub fn new<T>(location: Location<'src>, message: T) -> Self
  where
    T: Into<Cow<'static, str>>,
  {
    Self {
      location,
      message: message.into(),
    }
  }

  pub fn wrap_message<T>(self, msg: T) -> ParseError<'src>
  where
    T: Into<Cow<'static, str>>,
  {
    let wrapped_msg = format!("{}: {}", msg.into(), self.message.as_ref());
    self.sub_message(Cow::Owned(wrapped_msg))
  }

  pub fn sub_message<T>(self, msg: T) -> ParseError<'src>
  where
    T: Into<Cow<'static, str>>,
  {
    ParseError {
      location: self.location,
      message: msg.into(),
    }
  }
}

#[derive(Debug, Clone)]
pub struct ParseSuccess<'src, T>
where
  T: std::fmt::Debug + Clone,
{
  pub value: T,
  pub span: Span<'src>,
  pub src: ParseSrc<'src>,
}

impl<'src, T> ParseSuccess<'src, T>
where
  T: std::fmt::Debug + Clone,
{
  pub fn map<F, U>(self, f: F) -> ParseSuccess<'src, U>
  where
    U: std::fmt::Debug + Clone,
    F: FnOnce(T, Span<'src>) -> U,
  {
    ParseSuccess {
      value: f(self.value, self.span),
      span: self.span,
      src: self.src,
    }
  }
}

pub type ParseResult<'src, T> = Result<ParseSuccess<'src, T>, ParseError<'src>>;
