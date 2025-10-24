use crate::parse::{
  basic::{identifiers::parse_identifier, str::parse_char},
  data::{
    result::{ParseResult, ParseSuccess},
    src::ParseSrc,
  },
  expressions::common::Expression,
};

// This parses variable accesses.

pub fn parse_variable<'str>(
  mut src: ParseSrc<'str>,
) -> ParseResult<'str, Expression<'str>> {
  let start = src.clone();

  let marker = parse_char('$', src)?;
  src = marker.src;

  let name = parse_identifier(src)
    .map_err(|e| e.wrap_message("expected a variable name immediately after `$`"))?;

  src = name.src;

  Ok(ParseSuccess {
    value: Expression::Variable(name.value),
    span: src.span_from(&start),
    src,
  })
}

#[cfg(test)]
mod tests {
  use crate::parse::{
    data::src::ParseSrc,
    expressions::{common::Expression, variable::parse_variable},
  };

  #[test]
  fn test_parse_variable() {
    // Good EOF.
    let src = ParseSrc::new(None, "$foo");
    let result = parse_variable(src).expect("parse error");
    assert_eq!(result.value, Expression::Variable("foo"));
    assert_eq!(result.src.remaining_str(), "");

    // Good.
    let src = ParseSrc::new(None, "$alice bob");
    let result = parse_variable(src).expect("parse error");
    assert_eq!(result.value, Expression::Variable("alice"));
    assert_eq!(result.src.remaining_str(), " bob");

    // Different characters.
    let src = ParseSrc::new(None, "abc1");
    let _ = parse_variable(src).expect_err("parse not error");
  }
}
