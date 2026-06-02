use crate::{
  common::StringKind::{AngleBracketed, DoubleQuoted},
  parse::{
    ast::{Expression, ExpressionKind},
    data::{
      result::{ParseError, ParseResult, ParseSuccess},
      src::ParseSrc,
    },
    util::str::parse_char,
  },
};

pub fn parse_double_quoted_string<'src>(
  mut src: ParseSrc<'src>,
) -> ParseResult<'src, Expression<'src>> {
  let start = src.clone();
  let mut accum = String::new();

  src = parse_char('"', src)?.src;

  while let Some(next) = src.next() {
    if next == '"' {
      break;
    } else if next == '\\' {
      return Err(ParseError::new(
        start.location,
        format!("currently, backslashes are not considered valid in strings."),
      ));
    }

    accum.push(next);
  }

  let span = src.span_from(&start);
  Ok(ParseSuccess {
    value: Expression::new(ExpressionKind::String(DoubleQuoted(accum)), span),
    span,
    src,
  })
}

pub fn parse_angle_bracketed_string<'src>(
  mut src: ParseSrc<'src>,
) -> ParseResult<'src, Expression<'src>> {
  let start = src.clone();
  let mut accum = String::new();

  src = parse_char('<', src)?.src;

  while let Some(next) = src.next() {
    if next == '>' {
      break;
    } else if next == '\\' {
      return Err(ParseError::new(
        start.location,
        format!("currently, backslashes are not considered valid in strings."),
      ));
    }

    accum.push(next);
  }

  let span = src.span_from(&start);
  Ok(ParseSuccess {
    value: Expression::new(ExpressionKind::String(AngleBracketed(accum)), span),
    span,
    src,
  })
}

#[cfg(test)]
mod tests {
  use crate::parse::{
    ast::string,
    data::src::ParseSrc,
    expressions::string::{parse_angle_bracketed_string, parse_double_quoted_string},
  };

  #[test]
  fn test_parse_double_quoted_string() {
    // Good EOF.
    let src = ParseSrc::new(None, "\"vecs.h\"");
    let result = parse_double_quoted_string(src).expect("parse error");
    assert_eq!(result.value, string!("", "vecs.h"));
    assert_eq!(result.src.remaining_str(), "");

    // Good.
    let src = ParseSrc::new(None, "\"a b c d\" 6789");
    let result = parse_double_quoted_string(src).expect("parse error");
    assert_eq!(result.value, string!("", "a b c d"));
    assert_eq!(result.src.remaining_str(), " 6789");

    // Backslash.
    let src = ParseSrc::new(None, "\"ab\\c1\"");
    let _ = parse_double_quoted_string(src).expect_err("parse not error");

    // Different characters.
    let src = ParseSrc::new(None, "abc1");
    let _ = parse_double_quoted_string(src).expect_err("parse not error");
  }

  #[test]
  fn test_parse_angle_bracketed_string() {
    // Good EOF.
    let src = ParseSrc::new(None, "<vecs.h>");
    let result = parse_angle_bracketed_string(src).expect("parse error");
    assert_eq!(result.value, string!(<>, "vecs.h"));
    assert_eq!(result.src.remaining_str(), "");

    // Good.
    let src = ParseSrc::new(None, "<a b c d> 6789");
    let result = parse_angle_bracketed_string(src).expect("parse error");
    assert_eq!(result.value, string!(<>, "a b c d"));
    assert_eq!(result.src.remaining_str(), " 6789");

    // Backslash.
    let src = ParseSrc::new(None, "<ab\\c1>");
    let _ = parse_double_quoted_string(src).expect_err("parse not error");

    // Different characters.
    let src = ParseSrc::new(None, "abc1");
    let _ = parse_angle_bracketed_string(src).expect_err("parse not error");
  }
}
