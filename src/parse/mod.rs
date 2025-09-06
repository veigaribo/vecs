mod basic;
pub mod comments;
pub mod components;
pub mod data;
pub mod events;
pub mod systems;

use bumpalo::{Bump, collections::Vec};

use crate::parse::{
  basic::str::parse_whitespace,
  comments::parse_comment,
  components::{Component, parse_component},
  data::{
    result::{ParseError, ParseResult, ParseSuccess},
    src::ParseSrc,
  },
  events::{Event, parse_event},
  systems::{System, parse_system},
};

fn strip_comments(arena: &Bump, t: &mut str) {
  let mut src = ParseSrc::from(&*t);
  let mut spans = Vec::<(usize, usize)>::new_in(arena); // Just start and end bytes.

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
          Some(c) => {}
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
  pub components: Vec<'str, Component<'str>>,
  pub events: Vec<'str, Event<'str>>,
  pub systems: Vec<'str, System<'str>>,
}

impl<'str> Parsed<'str> {
  pub fn new(arena: &'str Bump) -> Self {
    Self {
      components: Vec::new_in(arena),
      events: Vec::new_in(arena),
      systems: Vec::new_in(arena),
    }
  }
}

pub fn parse<'str>(
  arena: &'str Bump,
  mut src: ParseSrc<'str>,
) -> ParseResult<'str, Parsed<'str>> {
  let start = src.clone();
  let mut parsed = Parsed::new(arena);

  let ParseSuccess { mut src, .. } = parse_whitespace(src)?;

  loop {
    if let Ok(success) = parse_component(arena, src.clone()) {
      parsed.components.push(success.value);
      src = success.src;
    } else if let Ok(success) = parse_event(arena, src.clone()) {
      parsed.events.push(success.value);
      src = success.src;
    } else if let Ok(success) = parse_system(arena, src.clone()) {
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

      let start_location = src.location;

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

    let ParseSuccess { src: item_src, .. } = parse_whitespace(src)?;
    src = item_src;
  }

  Ok(ParseSuccess {
    value: parsed,
    span: src.span_from(&start),
    src,
  })
}

#[cfg(test)]
mod tests {
  use bumpalo::Bump;

  use crate::parse::{
    Parsed,
    components::{Component, ComponentField},
    data::src::ParseSrc,
    events::{Event, EventField},
    parse, strip_comments,
    systems::System,
  };

  #[test]
  fn test_strip_comments() {
    let arena = Bump::new();
    let mut src_str = String::from(
      "
// this component is a component
component airton {\r\nint x;
  // believe it or not 
  int y; // letter Y <
}
",
    );

    let mut target_str = String::from(
      "
                                
component airton {\r\nint x;
                       
  int y;              
}
",
    );

    strip_comments(&arena, &mut src_str);
    assert_eq!(&src_str, &target_str);
  }

  #[test]
  fn test_parse() {
    let arena = Bump::new();

    let mut src = ParseSrc::from(
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

    let result = parse(&arena, src).expect("parse error");
    assert_eq!(
      result.value,
      Parsed {
        components: bumpalo::vec![
          in &arena;
          Component::new("transform",
            bumpalo::vec![in &arena;
              ComponentField::new("double", "x"),
              ComponentField::new("double", "y"),
            ]
          ),
          Component::new("render",
            bumpalo::vec![in &arena;
              ComponentField::new("texture_t", "texture"),
            ]
          ),
        ],

        events: bumpalo::vec![
          in &arena;
          Event::new("mouse_click",
            bumpalo::vec![in &arena;
              EventField::new("double", "x"),
              EventField::new("double", "y"),
              EventField::new("uint8_t", "button"),
            ]
          ),
        ],

        systems: bumpalo::vec![
          in &arena;
          System::new("move", bumpalo::vec![in &arena; "transform"]),
          System::new("render", bumpalo::vec![in &arena; "transform", "render"]),
        ]
      }
    );
  }
}
