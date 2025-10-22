use crate::parse::{
  basic::{
    identifiers::{is_identifier, parse_identifier},
    str::{parse_char, parse_str, parse_whitespace},
  },
  data::{
    result::{ParseResult, ParseSuccess},
    src::ParseSrc,
  },
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ComponentField<'str> {
  pub typ: &'str str,
  pub name: &'str str,
}

impl<'str> ComponentField<'str> {
  pub fn new(typ: &'str str, name: &'str str) -> Self {
    debug_assert!(is_identifier(typ));
    debug_assert!(is_identifier(name));

    Self { typ, name }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Component<'str> {
  pub name: &'str str,
  pub fields: Vec<ComponentField<'str>>,
}

impl<'str> Component<'str> {
  pub fn new(name: &'str str, fields: Vec<ComponentField<'str>>) -> Self {
    Self { name, fields }
  }
}

pub fn parse_component<'str>(
  mut src: ParseSrc<'str>,
) -> ParseResult<'str, Component<'str>> {
  let start = src.clone();
  let mut fields = Vec::<ComponentField>::new();

  src = parse_str("component", src)?.src;
  src = parse_whitespace(src)?.src;

  let parsed_name = parse_identifier(src)?;
  src = parsed_name.src;

  src = parse_whitespace(src)?.src;
  src = parse_char('{', src)?.src;
  src = parse_whitespace(src)?.src;

  while let Ok(success) = parse_component_field(src.clone()) {
    fields.push(success.value);
    src = parse_whitespace(success.src)?.src;
  }

  src = parse_whitespace(src)?.src;
  src = parse_char('}', src)?.src;

  Ok(ParseSuccess {
    value: Component::new(parsed_name.value, fields),
    span: src.span_from(&start),
    src,
  })
}

pub fn parse_component_field<'str>(
  mut src: ParseSrc<'str>,
) -> ParseResult<'str, ComponentField<'str>> {
  let start = src.clone();

  let parsed_type = parse_identifier(src)?;
  src = parsed_type.src;

  src = parse_whitespace(src)?.src;

  let parsed_name = parse_identifier(src)?;
  src = parsed_name.src;

  src = parse_whitespace(src)?.src;
  src = parse_char(';', src)?.src;

  Ok(ParseSuccess {
    value: ComponentField::new(parsed_type.value, parsed_name.value),
    span: src.span_from(&start),
    src,
  })
}

#[cfg(test)]
mod tests {
  use crate::parse::{
    components::{parse_component, parse_component_field, Component, ComponentField},
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
    let _ = parse_component_field(src).expect_err("parse not error");
  }

  #[test]
  fn test_parse_component() {
    // Good.
    let src = ParseSrc::new(
      None,
      "component vec2 {\
      double x; \
      double y; \
    } // math",
    );

    let result = parse_component(src).expect("parse error");
    assert_eq!(
      result.value,
      Component::new(
        "vec2",
        vec![
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

    let result = parse_component(src).expect("parse error");
    assert_eq!(result.value, Component::new("vec0", vec![]));
    assert_eq!(result.src.remaining_str(), " // a");

    // Different characters.
    let src = ParseSrc::new(None, ";abc");
    let _ = parse_component(src).expect_err("parse not error");
  }
}
