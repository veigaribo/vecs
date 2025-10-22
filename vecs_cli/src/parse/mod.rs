pub mod basic;
pub mod comments;
pub mod components;
pub mod data;
pub mod events;
pub mod systems;

use crate::parse::{
  basic::str::parse_whitespace,
  comments::parse_comment,
  components::{parse_component, Component},
  data::{
    result::{ParseError, ParseResult, ParseSuccess},
    src::ParseSrc,
  },
  events::{parse_event, Event},
  systems::{parse_system, System},
};

pub fn strip_comments(t: &mut str) {
  let mut src = ParseSrc::from(&*t);
  let mut spans = Vec::<(usize, usize)>::new(); // Just start and end bytes.

  loop {
    match parse_comment(src.clone()) {
      Ok(success) => {
        src = success.src;

        let span = success.span;
        spans.push((span.start.byte_offset, span.end.byte_offset));
      }
      Err(_) => {
        let c = src.next();

        match c {
          Some(_) => {}
          None => break,
        }
      }
    }
  }

  for (start, end) in spans {
    // SAFETY: Start and end come from `Location`s, which always advances in UTF-8
    // code points and so always lands in valid boundaries.
    unsafe {
      for byte in t[start..end].as_bytes_mut() {
        *byte = b' ';
      }
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parsed<'str> {
  pub components: Vec<Component<'str>>,
  pub events: Vec<Event<'str>>,
  pub systems: Vec<System<'str>>,
}

impl<'str> Parsed<'str> {
  pub fn new() -> Self {
    Self {
      components: Vec::new(),
      events: Vec::new(),
      systems: Vec::new(),
    }
  }
}

pub fn parse<'str>(mut src: ParseSrc<'str>) -> ParseResult<'str, Parsed<'str>> {
  let start = src.clone();
  let mut parsed = Parsed::new();

  src = parse_whitespace(src)?.src;

  loop {
    if let Ok(success) = parse_component(src.clone()) {
      parsed.components.push(success.value);
      src = success.src;
    } else if let Ok(success) = parse_event(src.clone()) {
      parsed.events.push(success.value);
      src = success.src;
    } else if let Ok(success) = parse_system(src.clone()) {
      parsed.systems.push(success.value);
      src = success.src;
    } else {
      if src.is_empty() {
        return Ok(ParseSuccess {
          value: parsed,
          span: src.span_from(&start),
          src,
        });
      }

      // The line or 200 bytes, whichever ends first.
      let found = (0..200)
        .into_iter()
        .zip(src.clone().take_while(|c| *c != '\n'))
        .map(|(_, c)| c)
        .collect::<String>();

      return Err(ParseError::new(
        src.location,
        format!("expected component, event or system, but found: {}", found),
      ));
    }

    src = parse_whitespace(src)?.src;
  }
}

#[cfg(test)]
mod tests {
  use crate::parse::{
    components::{Component, ComponentField},
    data::src::ParseSrc,
    events::{Event, EventField},
    parse, strip_comments,
    systems::System,
    Parsed,
  };

  #[test]
  fn test_strip_comments() {
    let mut src_str = String::from(
      "
// this component is a component
component airton {\r\nint x;
  // believe it or not 
  int y; // letter Y <
}
",
    );

    let target_str = String::from(
      "
                                
component airton {\r\nint x;
                       
  int y;              
}
",
    );

    strip_comments(&mut src_str);
    assert_eq!(&src_str, &target_str);
  }

  #[test]
  fn test_parse() {
    let src = ParseSrc::from(
      " \
      component transform { \
        double x; \
        double y; \
      } \
      \
      component render { \
        texture_t texture; \
      } \
      \
      event mouse_click { \
        double x; \
        double y; \
        uint8_t button; \
      } \
      \
      system move(transform) \
      system render(transform, render) \
      ",
    );

    let result = parse(src).expect("parse error");
    assert_eq!(
      result.value,
      Parsed {
        components: vec![
          Component::new(
            "transform",
            vec![
              ComponentField::new("double", "x"),
              ComponentField::new("double", "y"),
            ]
          ),
          Component::new(
            "render",
            vec![ComponentField::new("texture_t", "texture"),]
          ),
        ],

        events: vec![Event::new(
          "mouse_click",
          vec![
            EventField::new("double", "x"),
            EventField::new("double", "y"),
            EventField::new("uint8_t", "button"),
          ]
        ),],

        systems: vec![
          System::new("move", vec!["transform"]),
          System::new("render", vec!["transform", "render"]),
        ]
      }
    );
  }
}
