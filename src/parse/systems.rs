use bumpalo::{Bump, collections::Vec};

use crate::parse::{
  basic::{
    identifiers::parse_identifier,
    str::{parse_char, parse_str, parse_whitespace},
  },
  data::{
    result::{ParseResult, ParseSuccess},
    src::ParseSrc,
  },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct System<'str> {
  name: &'str str,
  params: Vec<'str, &'str str>,
}

impl<'str> System<'str> {
  pub fn new(name: &'str str, params: Vec<'str, &'str str>) -> Self {
    Self { name, params }
  }
}

pub fn parse_system<'str>(
  arena: &'str Bump,
  mut src: ParseSrc<'str>,
) -> ParseResult<'str, System<'str>> {
  let start = src.clone();
  let mut params = Vec::<&'str str>::new_in(arena);

  src = parse_str("system", src)?.src;
  src = parse_whitespace(src)?.src;

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
        value: System::new(parsed_name.value, params),
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
    value: System::new(parsed_name.value, params),
    span: src.span_from(&start),
    src,
  })
}

#[cfg(test)]
mod tests {
  use bumpalo::Bump;

  use crate::parse::{
    data::src::ParseSrc,
    systems::{System, parse_system},
  };

  #[test]
  fn test_parse_system() {
    let arena = Bump::new();

    // Good.
    let src = ParseSrc::new(None, "system render(transform, render) // b");
    let result = parse_system(&arena, src).expect("parse error");
    assert_eq!(
      result.value,
      System::new("render", bumpalo::vec![in &arena; "transform", "render"])
    );
    assert_eq!(result.src.remaining_str(), " // b");

    // Good. No components.
    let src = ParseSrc::new(None, "system render() // b");
    let result = parse_system(&arena, src).expect("parse error");
    assert_eq!(
      result.value,
      System::new("render", bumpalo::vec![in &arena;])
    );
    assert_eq!(result.src.remaining_str(), " // b");

    // Good. One component.
    let src = ParseSrc::new(None, "system move(transform) // b");
    let result = parse_system(&arena, src).expect("parse error");
    assert_eq!(
      result.value,
      System::new("move", bumpalo::vec![in &arena; "transform"])
    );
    assert_eq!(result.src.remaining_str(), " // b");

    // Good. Trailing comma.
    let src = ParseSrc::new(None, "system render(transform, render,) // b");
    let result = parse_system(&arena, src).expect("parse error");
    assert_eq!(
      result.value,
      System::new("render", bumpalo::vec![in &arena; "transform", "render"])
    );
    assert_eq!(result.src.remaining_str(), " // b");

    // Different characters.
    let src = ParseSrc::new(None, ";abc");
    let _ = parse_system(&arena, src).expect_err("parse not error");
  }
}
