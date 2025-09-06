use bumpalo::{Bump, collections::Vec};

use crate::parse::{
  basic::{
    identifiers::parse_identifier,
    str::{parse_char, parse_str, parse_whitespace},
  },
  data::{
    result::{ParseError, ParseResult, ParseSuccess},
    src::ParseSrc,
  },
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ComponentField<'str> {
  typ: &'str str,
  name: &'str str,
}

impl<'str> ComponentField<'str> {
  pub fn new(typ: &'str str, name: &'str str) -> Self {
    Self { typ, name }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Component<'str> {
  name: &'str str,
  fields: Vec<'str, ComponentField<'str>>,
}

impl<'str> Component<'str> {
  pub fn new(name: &'str str, fields: Vec<'str, ComponentField<'str>>) -> Self {
    Self { name, fields }
  }
}

pub fn parse_component<'str>(
  arena: &'str Bump,
  mut src: ParseSrc<'str>,
) -> ParseResult<'str, Component<'str>> {
  let start = src.clone();
  let mut fields = Vec::<ComponentField>::new_in(arena);

  let ParseSuccess { mut src, .. } = parse_str("component", src)?;
  let ParseSuccess { mut src, .. } = parse_whitespace(src)?;

  let ParseSuccess {
    mut src,
    value: name,
    ..
  } = parse_identifier(src)?;

  let ParseSuccess { mut src, .. } = parse_whitespace(src)?;
  let ParseSuccess { mut src, .. } = parse_char('{', src)?;
  let ParseSuccess { mut src, .. } = parse_whitespace(src)?;

  while let Ok(success) = parse_component_field(src.clone()) {
    fields.push(success.value);
    let item_src = success.src;

    let ParseSuccess { src: item_src, .. } = parse_whitespace(item_src)?;
    src = item_src;
  }

  let ParseSuccess { mut src, .. } = parse_whitespace(src)?;
  let ParseSuccess { mut src, .. } = parse_char('}', src)?;

  Ok(ParseSuccess {
    value: Component::new(name, fields),
    span: src.span_from(&start),
    src,
  })
}

pub fn parse_component_field<'str>(
  mut src: ParseSrc<'str>,
) -> ParseResult<'str, ComponentField<'str>> {
  let start = src.clone();

  let ParseSuccess {
    mut src,
    value: typ,
    ..
  } = parse_identifier(src)?;

  let ParseSuccess { mut src, .. } = parse_whitespace(src)?;

  let ParseSuccess {
    mut src,
    value: name,
    ..
  } = parse_identifier(src)?;

  let ParseSuccess { mut src, .. } = parse_whitespace(src)?;
  let ParseSuccess { mut src, .. } = parse_char(';', src)?;

  Ok(ParseSuccess {
    value: ComponentField::new(typ, name),
    span: src.span_from(&start),
    src,
  })
}

#[cfg(test)]
mod tests {
  use bumpalo::Bump;

  use crate::parse::{
    components::{Component, ComponentField, parse_component, parse_component_field},
    data::src::ParseSrc,
  };

  #[test]
  fn test_parse_component_field() {
    // Good.
    let src = ParseSrc::new(None, "int x; // a");
    let result = parse_component_field(src).expect("parse error");
    assert_eq!(result.value, ComponentField::new("int", "x"));
    assert_eq!(result.src.remaining_str(), " // a");

    // Different characters.
    let src = ParseSrc::new(None, ";abc");
    let result = parse_component_field(src).expect_err("parse not error");
  }

  #[test]
  fn test_parse_component() {
    let arena = Bump::new();

    // Good.
    let src = ParseSrc::new(
      None,
      "component vec2 {\
      double x; \
      double y; \
    } // math",
    );

    let result = parse_component(&arena, src).expect("parse error");
    assert_eq!(
      result.value,
      Component::new(
        "vec2",
        bumpalo::vec![
          in &arena;
          ComponentField::new("double", "x"),
          ComponentField::new("double", "y")
        ]
      )
    );
    assert_eq!(result.src.remaining_str(), " // math");

    // Good. No fields.
    let src = ParseSrc::new(
      None,
      "component vec0 {\
    } // a",
    );

    let result = parse_component(&arena, src).expect("parse error");
    assert_eq!(
      result.value,
      Component::new("vec0", bumpalo::vec![in &arena;])
    );
    assert_eq!(result.src.remaining_str(), " // a");

    // Different characters.
    let src = ParseSrc::new(None, ";abc");
    let result = parse_component(&arena, src).expect_err("parse not error");
  }
}
