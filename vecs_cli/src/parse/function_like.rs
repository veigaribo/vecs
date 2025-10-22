use crate::parse::{
  basic::{
    identifiers::parse_identifier,
    str::{parse_char, parse_whitespace},
  },
  data::{
    result::{ParseResult, ParseSuccess},
    src::ParseSrc,
  },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function<'str> {
  name: &'str str,
  params: Vec<&'str str>,
}

impl<'str> Function<'str> {
  pub fn new(name: &'str str, params: Vec<&'str str>) -> Self {
    Self { name, params }
  }
}

pub fn parse_function<'str>(
  mut src: ParseSrc<'str>,
) -> ParseResult<'str, Function<'str>> {
  let start = src.clone();
  let mut params = Vec::<&'str str>::new();

  let parsed_name = parse_identifier(src)?;
  src = parsed_name.src;

  src = parse_whitespace(src)?.src;
  src = parse_char('(', src)?.src;
  src = parse_whitespace(src)?.src;

  let first_param = parse_identifier(src.clone());

  match first_param {
    Ok(first_param) => {
      params.push(first_param.value);
      src = first_param.src;
    }
    Err(_) => {
      src = parse_char(')', src)?.src;

      return Ok(ParseSuccess {
        value: Function::new(parsed_name.value, params),
        span: src.span_from(&start),
        src,
      });
    }
  }

  while let Ok(success) = parse_char(',', src.clone()) {
    src = parse_whitespace(success.src)?.src;

    match parse_identifier(src.clone()) {
      Ok(ParseSuccess {
        src: identifier_src,
        value: param,
        ..
      }) => {
        params.push(param);
        src = parse_whitespace(identifier_src)?.src;
      }
      Err(_) => {
        break;
      }
    }
  }

  src = parse_whitespace(src)?.src;
  src = parse_char(')', src)?.src;

  Ok(ParseSuccess {
    value: Function::new(parsed_name.value, params),
    span: src.span_from(&start),
    src,
  })
}

#[cfg(test)]
mod tests {
  use crate::parse::{
    data::src::ParseSrc,
    function_like::{parse_function, Function},
  };

  #[test]
  fn test_parse_function() {
    // Good.
    let src = ParseSrc::new(None, "render(transform, render) // b");
    let result = parse_function(src).expect("parse error");
    assert_eq!(
      result.value,
      Function::new("render", vec!["transform", "render"])
    );
    assert_eq!(result.src.remaining_str(), " // b");

    // Good. No components.
    let src = ParseSrc::new(None, "render() // b");
    let result = parse_function(src).expect("parse error");
    assert_eq!(result.value, Function::new("render", vec![]));
    assert_eq!(result.src.remaining_str(), " // b");

    // Good. One component.
    let src = ParseSrc::new(None, "move(transform) // b");
    let result = parse_function(src).expect("parse error");
    assert_eq!(result.value, Function::new("move", vec!["transform"]));
    assert_eq!(result.src.remaining_str(), " // b");

    // Good. Trailing comma.
    let src = ParseSrc::new(None, "render(transform, render,) // b");
    let result = parse_function(src).expect("parse error");
    assert_eq!(
      result.value,
      Function::new("render", vec!["transform", "render"])
    );
    assert_eq!(result.src.remaining_str(), " // b");

    // Different characters.
    let src = ParseSrc::new(None, ";abc");
    let _ = parse_function(src).expect_err("parse not error");
  }
}
