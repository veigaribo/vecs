use crate::parse::{
  ast::Expression,
  data::{
    result::{ParseResult, ParseSuccess},
    src::ParseSrc,
  },
  expressions::parse_basic_expression,
  util::str::parse_whitespace,
};

pub fn parse_application<'str>(
  mut src: ParseSrc<'str>,
) -> ParseResult<'str, Expression<'str>> {
  let start = src.clone();

  // Unlikely for there to be more than that.
  let mut seq = Vec::<Expression>::with_capacity(4);

  let head = parse_basic_expression(src)?;

  src = parse_whitespace(head.src)?.src;
  seq.push(head.value);

  loop {
    if src.is_empty() {
      break;
    }

    let next = parse_basic_expression(src.clone());

    if let Ok(success) = next {
      src = parse_whitespace(success.src)?.src;
      seq.push(success.value);
    } else {
      break;
    }
  }

  Ok(ParseSuccess {
    value: Expression::Application(seq),
    span: src.span_from(&start),
    src,
  })
}

#[cfg(test)]
mod tests {
  use crate::parse::{
    ast::{app, int, list, sym, var},
    data::src::ParseSrc,
    expressions::application::parse_application,
  };

  #[test]
  fn test_parse_application() {
    // Good.
    let src = ParseSrc::new(None, "foo $bar 100 { ...{ baz } }; a");
    let expected = app![
      sym!("foo"),
      var!("bar"),
      int!(100),
      list![...app!(list![app!(sym!("baz"))])],
    ];

    let result = parse_application(src).expect("parse error");
    assert_eq!(result.value, expected);
    assert_eq!(result.src.remaining_str(), "; a");

    // Invalid.
    let src = ParseSrc::new(None, " abc1");
    let _ = parse_application(src).expect_err("parse not error");
  }
}
