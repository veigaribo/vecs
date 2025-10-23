use crate::parse::{
  basic::str::parse_str,
  data::{
    result::{ParseResult, ParseSuccess},
    src::ParseSrc,
  },
};

pub fn parse_inline_comment<'str>(
  mut src: ParseSrc<'str>,
) -> ParseResult<'str, &'str str> {
  let start = src.clone();

  src = parse_str("//", src)?.src;

  while let Some(content) = src.peek() {
    if content == '\n' {
      break;
    }

    src.next();
  }

  let span = src.span_from(&start);

  Ok(ParseSuccess {
    value: &src.slice(span)[2..].trim(),
    span,
    src,
  })
}

pub fn parse_multiline_comment<'str>(
  mut src: ParseSrc<'str>,
) -> ParseResult<'str, &'str str> {
  let start = src.clone();

  src = parse_str("/*", src)?.src;

  loop {
    let result = parse_str("*/", src.clone());

    if let Ok(success) = result {
      src = success.src;
      break;
    } else {
      src.next();
    }
  }

  let span = src.span_from(&start);
  let slice = src.slice(span);

  Ok(ParseSuccess {
    value: &slice[2..slice.len() - 2].trim(),
    span,
    src,
  })
}

pub fn parse_comment<'str>(src: ParseSrc<'str>) -> ParseResult<'str, &'str str> {
  parse_inline_comment(src.clone()).or_else(|_| parse_multiline_comment(src))
}

#[cfg(test)]
mod tests {
  use crate::parse::{
    comments::{parse_comment, parse_inline_comment, parse_multiline_comment},
    data::src::ParseSrc,
  };

  #[test]
  fn test_parse_inline_comment_collect() {
    // Good EOF.
    let src = ParseSrc::new(None, "// ab c");
    let result = parse_inline_comment(src).expect("parse error");
    assert_eq!(result.value, "ab c");
    assert_eq!(result.src.remaining_str(), "");

    // Good EOL.
    let src = ParseSrc::new(None, "// ab c\nd");
    let result = parse_inline_comment(src).expect("parse error");
    assert_eq!(result.value, "ab c");
    assert_eq!(result.src.remaining_str(), "\nd");

    // Different characters.
    let src = ParseSrc::new(None, "abc");
    let _ = parse_inline_comment(src).expect_err("parse not error");
  }

  #[test]
  fn test_parse_multiline_comment_collect() {
    // Good EOF.
    let src = ParseSrc::new(None, "/* ab c */");
    let result = parse_multiline_comment(src).expect("parse error");
    assert_eq!(result.value, "ab c");
    assert_eq!(result.src.remaining_str(), "");

    // Good EOL.
    let src = ParseSrc::new(None, "/* ab c\nd*/\ne");
    let result = parse_multiline_comment(src).expect("parse error");
    assert_eq!(result.value, "ab c\nd");
    assert_eq!(result.src.remaining_str(), "\ne");

    // Different characters.
    let src = ParseSrc::new(None, "abc");
    let _ = parse_multiline_comment(src).expect_err("parse not error");
  }

  #[test]
  fn test_parse_comment_collect() {
    // Inline.
    let src = ParseSrc::new(None, "// ab c\nx");
    let result = parse_comment(src).expect("parse error");
    assert_eq!(result.value, "ab c");
    assert_eq!(result.src.remaining_str(), "\nx");

    // Multiline.
    let src = ParseSrc::new(None, "/* ab c */\nx");
    let result = parse_comment(src).expect("parse error");
    assert_eq!(result.value, "ab c");
    assert_eq!(result.src.remaining_str(), "\nx");

    // Different characters.
    let src = ParseSrc::new(None, "abc");
    let _ = parse_comment(src).expect_err("parse not error");
  }
}
