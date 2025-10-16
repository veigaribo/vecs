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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct EventField<'str> {
  typ: &'str str,
  name: &'str str,
}

impl<'str> EventField<'str> {
  pub fn new(typ: &'str str, name: &'str str) -> Self {
    Self { typ, name }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event<'str> {
  name: &'str str,
  fields: Vec<'str, EventField<'str>>,
}

impl<'str> Event<'str> {
  pub fn new(name: &'str str, fields: Vec<'str, EventField<'str>>) -> Self {
    Self { name, fields }
  }
}

pub fn parse_event<'str>(
  arena: &'str Bump,
  mut src: ParseSrc<'str>,
) -> ParseResult<'str, Event<'str>> {
  let start = src.clone();
  let mut fields = Vec::<EventField>::new_in(arena);

  src = parse_str("event", src)?.src;
  src = parse_whitespace(src)?.src;

  let parsed_name = parse_identifier(src)?;
  src = parsed_name.src;

  src = parse_whitespace(src)?.src;
  src = parse_char('{', src)?.src;
  src = parse_whitespace(src)?.src;

  while let Ok(success) = parse_event_field(src.clone()) {
    fields.push(success.value);
    src = parse_whitespace(success.src)?.src;
  }

  src = parse_whitespace(src)?.src;
  src = parse_char('}', src)?.src;

  Ok(ParseSuccess {
    value: Event::new(parsed_name.value, fields),
    span: src.span_from(&start),
    src,
  })
}

pub fn parse_event_field<'str>(
  mut src: ParseSrc<'str>,
) -> ParseResult<'str, EventField<'str>> {
  let start = src.clone();

  let parsed_type = parse_identifier(src)?;
  src = parsed_type.src;

  src = parse_whitespace(src)?.src;

  let parsed_name = parse_identifier(src)?;
  src = parsed_name.src;

  src = parse_whitespace(src)?.src;
  src = parse_char(';', src)?.src;

  Ok(ParseSuccess {
    value: EventField::new(parsed_type.value, parsed_name.value),
    span: src.span_from(&start),
    src,
  })
}

#[cfg(test)]
mod tests {
  use bumpalo::Bump;

  use crate::parse::{
    data::src::ParseSrc,
    events::{Event, EventField, parse_event, parse_event_field},
  };

  #[test]
  fn test_parse_event_field() {
    // Good.
    let src = ParseSrc::new(None, "int x; // a");
    let result = parse_event_field(src).expect("parse error");
    assert_eq!(result.value, EventField::new("int", "x"));
    assert_eq!(result.src.remaining_str(), " // a");

    // Different characters.
    let src = ParseSrc::new(None, ";abc");
    let _ = parse_event_field(src).expect_err("parse not error");
  }

  #[test]
  fn test_parse_event() {
    let arena = Bump::new();

    // Good.
    let src = ParseSrc::new(
      None,
      "event mouse_move {\
      double x; \
      double y; \
    } // math",
    );

    let result = parse_event(&arena, src).expect("parse error");
    assert_eq!(
      result.value,
      Event::new(
        "mouse_move",
        bumpalo::vec![
          in &arena;
          EventField::new("double", "x"),
          EventField::new("double", "y")
        ]
      )
    );
    assert_eq!(result.src.remaining_str(), " // math");

    // Good. No fields.
    let src = ParseSrc::new(
      None,
      "event noop {\
    } // a",
    );

    let result = parse_event(&arena, src).expect("parse error");
    assert_eq!(result.value, Event::new("noop", bumpalo::vec![in &arena;]));
    assert_eq!(result.src.remaining_str(), " // a");

    // Different characters.
    let src = ParseSrc::new(None, ";abc");
    let _ = parse_event(&arena, src).expect_err("parse not error");
  }
}
