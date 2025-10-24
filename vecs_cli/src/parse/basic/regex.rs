use regex::{Match, Regex};

use crate::parse::data::{
  result::{ParseError, ParseResult, ParseSuccess},
  src::ParseSrc,
};

pub fn parse_regex<'str>(
  mut src: ParseSrc<'str>,
  regex: &Regex,
) -> ParseResult<'str, Match<'str>> {
  debug_assert!(
    regex.as_str().starts_with('^'),
    "regex must match on the start of the string (^) only: `{}`",
    regex.as_str()
  );

  let start = src.clone();

  let haystack = &src.src[src.location.byte_offset..];
  let maybe_match = regex.find(haystack);

  match maybe_match {
    Some(matsh) => {
      src.advance_bytes(matsh.len());
      let span = src.span_from(&start);

      Ok(ParseSuccess {
        value: matsh,
        span,
        src,
      })
    }
    None => {
      return Err(ParseError::new(
        start.location,
        format!(
          "expected to find a match of the regular expression `{}`, but didn't",
          regex.as_str()
        ),
      ));
    }
  }
}

#[cfg(test)]
mod tests {
  use regex::Regex;

  use crate::parse::{basic::regex::parse_regex, data::src::ParseSrc};

  #[test]
  fn test_parse_regex() {
    let regex = Regex::new("^[0-9]+").expect("regex error");

    // Good EOF.
    let src = ParseSrc::new(None, "12345");
    let result = parse_regex(src, &regex).expect("parse error");
    assert_eq!(result.value.as_str(), "12345");
    assert_eq!(result.src.remaining_str(), "");

    // Good.
    let src = ParseSrc::new(None, "12345 6789");
    let result = parse_regex(src, &regex).expect("parse error");
    assert_eq!(result.value.as_str(), "12345");
    assert_eq!(result.src.remaining_str(), " 6789");

    // Different characters.
    let src = ParseSrc::new(None, "abc1");
    let _ = parse_regex(src, &regex).expect_err("parse not error");
  }
}
