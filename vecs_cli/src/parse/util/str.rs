use crate::parse::data::{
  result::{ParseError, ParseResult, ParseSuccess},
  src::ParseSrc,
};

pub fn parse_str<'str>(
  target: &'str str,
  src: ParseSrc<'str>,
) -> ParseResult<'str, &'str str> {
  let start = src.clone();

  let target_iter = ParseSrc::from(target);
  let mut zip = src.zip(target_iter);

  while let Some((src_char, target_char)) = zip.next() {
    if src_char != target_char {
      let (src, _) = zip.get();

      let found_len = src.span_from(&start).len();
      let found = start.clone().take(found_len).collect::<String>();

      return Err(ParseError::new(
        start.location,
        format!("expected \"{}\", but found \"{}\"", target, found),
      ));
    }
  }

  let (src, mut target_iter) = zip.get();

  if target_iter.next() != None {
    let found_len = src.span_from(&start).len();
    let found = start.clone().take(found_len).collect::<String>();

    return Err(ParseError::new(
      start.location,
      format!(
        "expected \"{}\", but found \"{}\" then the source ended",
        target, found
      ),
    ));
  }

  Ok(ParseSuccess {
    value: target,
    span: src.span_from(&start),
    src,
  })
}

pub fn parse_char<'str>(
  target: char,
  mut src: ParseSrc<'str>,
) -> ParseResult<'str, char> {
  let start = src.clone();

  if let Some(next) = src.peek() {
    if next != target {
      let start_location = start.location;

      return Err(ParseError::new(
        start_location,
        format!("expected '{}', but found '{}'", target, next),
      ));
    }

    src.next();
  }

  Ok(ParseSuccess {
    value: target,
    span: src.span_from(&start),
    src,
  })
}

pub fn parse_whitespace<'str>(mut src: ParseSrc<'str>) -> ParseResult<'str, ()> {
  let start = src.clone();

  while let Some(next) = src.peek() {
    if !char::is_whitespace(next) {
      break;
    }

    src.next();
  }

  Ok(ParseSuccess {
    value: (),
    span: src.span_from(&start),
    src,
  })
}

#[cfg(test)]
mod tests {
  use crate::parse::{
    util::str::{parse_str, parse_whitespace},
    data::src::ParseSrc,
  };

  #[test]
  fn test_parse_str() {
    // Good.
    let src = ParseSrc::new(None, "abcd");
    let result = parse_str("abc", src).expect("parse error");
    assert_eq!(result.value, "abc");
    assert_eq!(result.src.remaining_str(), "d");

    // Different characters.
    let src = ParseSrc::new(None, "abc");
    let _ = parse_str("def", src).expect_err("parse not error");

    // Source too short.
    let src = ParseSrc::new(None, "ab");
    let _ = parse_str("abc", src).expect_err("parse not error");
  }

  #[test]
  fn test_parse_whitespace() {
    // Good.
    let src = ParseSrc::new(None, "  a");
    let result = parse_whitespace(src).expect("parse error");
    assert_eq!(result.src.remaining_str(), "a");

    // Different characters.
    let src = ParseSrc::new(None, "abc");
    let result = parse_whitespace(src).expect("parse error");
    assert_eq!(result.src.remaining_str(), "abc");
  }
}
