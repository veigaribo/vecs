use std::borrow::Cow;

use crate::parse::data::{src::ParseSrc, str::Location};

use super::str::Span;

#[derive(Debug, Clone)]
pub struct ParseError<'str> {
  location: Location<'str>,
  message: Cow<'static, str>,
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
