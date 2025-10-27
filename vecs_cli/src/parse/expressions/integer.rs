use std::sync::LazyLock;

use regex::Regex;

use crate::parse::{
  ast::Expression,
  data::{
    result::{ParseResult, ParseSuccess},
    src::ParseSrc,
  },
  util::regex::parse_regex,
};

static INTEGER_REGEX: LazyLock<Regex> =
  LazyLock::new(|| Regex::new("^[0-9][0-9_]*").expect("integer regex error"));

pub fn parse_integer<'str>(
  mut src: ParseSrc<'str>,
) -> ParseResult<'str, Expression<'str>> {
  let start = src.clone();

  let matsh = parse_regex(src, &INTEGER_REGEX)?;
  src = matsh.src;

  let mut accum: i128 = 0;

  for digit in matsh.value.as_str().bytes() {
    if digit == b'_' {
      continue;
    }

    accum = accum * 10 + ((digit - b'0') as i128);
  }

  Ok(ParseSuccess {
    value: Expression::Integer(accum),
    span: src.span_from(&start),
    src,
  })
}

#[cfg(test)]
mod tests {
  use crate::parse::{
    ast::int, data::src::ParseSrc, expressions::integer::parse_integer,
  };

  #[test]
  fn test_parse_integer() {
    // Good EOF.
    let src = ParseSrc::new(None, "12_345");
    let result = parse_integer(src).expect("parse error");
    assert_eq!(result.value, int!(12345));
    assert_eq!(result.src.remaining_str(), "");

    // Good.
    let src = ParseSrc::new(None, "12345 6789");
    let result = parse_integer(src).expect("parse error");
    assert_eq!(result.value, int!(12345));
    assert_eq!(result.src.remaining_str(), " 6789");

    // Different characters.
    let src = ParseSrc::new(None, "abc1");
    let _ = parse_integer(src).expect_err("parse not error");
  }
}
