use crate::parse::{
  ast::{Expression, ListEntry},
  data::{
    result::{ParseResult, ParseSuccess},
    src::ParseSrc,
  },
  expressions::parse_expression,
  util::str::{parse_char, parse_str, parse_whitespace},
};

pub fn parse_list<'str>(
  mut src: ParseSrc<'str>,
) -> ParseResult<'str, Expression<'str>> {
  let start = src.clone();

  let mut list = Vec::<ListEntry>::new();

  let marker = parse_char('{', src)?;
  src = parse_whitespace(marker.src)?.src;

  // Check if empty.
  let maybe_finish = parse_char('}', src.clone());

  if let Ok(finish) = maybe_finish {
    src = finish.src;
    list.shrink_to_fit();

    return Ok(ParseSuccess {
      value: Expression::List(list),
      span: src.span_from(&start),
      src,
    });
  }

  loop {
    let embed_marker = parse_str("...", src.clone());

    let is_embedding = if let Ok(success) = embed_marker {
      src = parse_whitespace(success.src)?.src;
      true
    } else {
      false
    };

    let expr = parse_expression(src)?;
    src = parse_whitespace(expr.src)?.src;

    let entry = if is_embedding {
      ListEntry::Embed(expr.value)
    } else {
      ListEntry::Expr(expr.value)
    };

    list.push(entry);

    let sep = parse_char(';', src.clone()).or_else(|_| parse_char(',', src.clone()));

    if let Ok(success) = sep {
      src = parse_whitespace(success.src)?.src;
    }

    let maybe_finish = parse_char('}', src.clone());

    if let Ok(finish) = maybe_finish {
      src = finish.src;
      break;
    }
  }

  Ok(ParseSuccess {
    value: Expression::List(list),
    span: src.span_from(&start),
    src,
  })
}

#[cfg(test)]
mod tests {
  use crate::parse::{
    ast::{app, int, list, sym},
    data::src::ParseSrc,
    expressions::list::parse_list,
  };

  #[test]
  fn test_parse_list() {
    // Good empty.
    let src = ParseSrc::new(None, "{} a");
    let expected = list![];

    let result = parse_list(src).expect("parse error");
    assert_eq!(result.value, expected);
    assert_eq!(result.src.remaining_str(), " a");

    // Good comma.
    let src = ParseSrc::new(None, "{ foo, bar } a");
    let expected = list![app!(sym!("foo")), app!(sym!("bar"))];

    let result = parse_list(src).expect("parse error");
    assert_eq!(result.value, expected);
    assert_eq!(result.src.remaining_str(), " a");

    // Good comma trailing.
    let src = ParseSrc::new(None, "{ foo, bar, } a");
    let expected = list![app!(sym!("foo")), app!(sym!("bar"))];

    let result = parse_list(src).expect("parse error");
    assert_eq!(result.value, expected);
    assert_eq!(result.src.remaining_str(), " a");

    // Good semicolon.
    let src = ParseSrc::new(None, "{ foo; bar; } a");
    let expected = list![app!(sym!("foo")), app!(sym!("bar"))];

    let result = parse_list(src).expect("parse error");
    assert_eq!(result.value, expected);
    assert_eq!(result.src.remaining_str(), " a");

    // Good semicolon trailing.
    let src = ParseSrc::new(None, "{ foo; bar; } a");
    let expected = list![app!(sym!("foo")), app!(sym!("bar"))];

    let result = parse_list(src).expect("parse error");
    assert_eq!(result.value, expected);
    assert_eq!(result.src.remaining_str(), " a");

    // Good embed.
    let src = ParseSrc::new(None, "{ 1, 2, ...{ 3, 4 }, } a");
    let expected = list![
      app!(int!(1)),
      app!(int!(2)),
      ...app!(list![app!(int!(3)), app!(int!(4))]),
    ];

    let result = parse_list(src).expect("parse error");
    assert_eq!(result.value, expected);
    assert_eq!(result.src.remaining_str(), " a");

    // Invalid.
    let src = ParseSrc::new(None, "abc1");
    let _ = parse_list(src).expect_err("parse not error");

    // Invalid: Lone comma.
    let src = ParseSrc::new(None, "{ , }");
    let _ = parse_list(src).expect_err("parse not error");
  }
}
