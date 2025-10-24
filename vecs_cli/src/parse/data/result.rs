use std::{borrow::Cow, fmt::Write};

use crate::parse::data::{src::ParseSrc, str::Location};

use super::str::Span;

#[derive(Debug, Clone)]
pub struct ParseError<'str> {
  location: Location<'str>,
  message: Cow<'static, str>,
}

impl<'str> std::fmt::Display for ParseError<'str> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.location.fmt(f)?;
    f.write_char(':')?;
    f.write_str(self.message.as_ref())?;
    Ok(())
  }
}

impl<'str> ParseError<'str> {
  pub fn new<T>(location: Location<'str>, message: T) -> Self
  where
    T: Into<Cow<'static, str>>,
  {
    Self {
      location,
      message: message.into(),
    }
  }

  pub fn wrap_message<T>(self, msg: T) -> ParseError<'str>
  where
    T: Into<Cow<'static, str>>,
  {
    let wrapped_msg = format!("{}: {}", msg.into(), self.message.as_ref());
    self.sub_message(Cow::Owned(wrapped_msg))
  }

  pub fn sub_message<T>(self, msg: T) -> ParseError<'str>
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
pub struct ParseSuccess<'str, T>
where
  T: std::fmt::Debug + Clone,
{
  pub value: T,
  pub span: Span<'str>,
  pub src: ParseSrc<'str>,
}

impl<'str, T> ParseSuccess<'str, T>
where
  T: std::fmt::Debug + Clone,
{
  pub fn map<F, U>(self, f: F) -> ParseSuccess<'str, U>
  where
    U: std::fmt::Debug + Clone,
    F: FnOnce(T) -> U,
  {
    ParseSuccess {
      value: f(self.value),
      span: self.span,
      src: self.src,
    }
  }
}

pub type ParseResult<'str, T> = Result<ParseSuccess<'str, T>, ParseError<'str>>;
