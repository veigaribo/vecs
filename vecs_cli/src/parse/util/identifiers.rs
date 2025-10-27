use crate::parse::data::{
  result::{ParseError, ParseResult, ParseSuccess},
  src::ParseSrc,
};

pub fn parse_identifier<'str>(
  mut src: ParseSrc<'str>,
) -> ParseResult<'str, &'str str> {
  let start = src.clone();
  let first = src.next();

  if let Some(first) = first {
    if !(first.is_ascii_alphabetic() || first == '_') {
      return Err(ParseError::new(
        start.location,
        format!(
          "expected an identifier, but '{}' is not a valid identifier start",
          first,
        ),
      ));
    }
  } else {
    return Err(ParseError::new(
      start.location,
      format!("expected an identifier, but the source ended",),
    ));
  }

  while let Some(next) = src.peek() {
    if !(next.is_ascii_alphanumeric() || next == '_') {
      break;
    }

    src.next();
  }

  let span = src.span_from(&start);

  Ok(ParseSuccess {
    value: &src.slice(span).trim(),
    span,
    src,
  })
}

#[cfg(test)]
mod tests {
  use crate::parse::{data::src::ParseSrc, util::identifiers::parse_identifier};

  #[test]
  fn test_parse_identifier() {
    // Good EOF.
    let src = ParseSrc::new(None, "ab_c1");
    let result = parse_identifier(src).expect("parse error");
    assert_eq!(result.value, "ab_c1");
    assert_eq!(result.src.remaining_str(), "");

    // Good.
    let src = ParseSrc::new(None, "_ab_c1 x");
    let result = parse_identifier(src).expect("parse error");
    assert_eq!(result.value, "_ab_c1");
    assert_eq!(result.src.remaining_str(), " x");

    // Different characters.
    let src = ParseSrc::new(None, "1abc");
    let _ = parse_identifier(src).expect_err("parse not error");
  }
}
