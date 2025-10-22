pub mod basic;
pub mod comments;
pub mod data;
pub mod function_like;
pub mod struct_like;

use crate::parse::{
  basic::{identifiers::parse_identifier, str::parse_whitespace},
  comments::parse_comment,
  data::{
    result::{ParseError, ParseResult, ParseSuccess},
    src::ParseSrc,
  },
  function_like::{parse_function, Function},
  struct_like::{parse_struct, Struct},
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
  pub components: Vec<Struct<'str>>,
  pub events: Vec<Struct<'str>>,
  pub systems: Vec<Function<'str>>,
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
    if src.is_empty() {
      return Ok(ParseSuccess {
        value: parsed,
        span: src.span_from(&start),
        src,
      });
    }

    let tag = parse_identifier(src)?;
    src = tag.src;
    src = parse_whitespace(src)?.src;

    match tag.value {
      "component" => {
        let success = parse_struct(src.clone())?;
        parsed.components.push(success.value);
        src = success.src;
      }
      "event" => {
        let success = parse_struct(src.clone())?;
        parsed.events.push(success.value);
        src = success.src;
      }
      "system" => {
        let success = parse_function(src.clone())?;
        parsed.systems.push(success.value);
        src = success.src;
      }
      unrecognized => {
        return Err(ParseError::new(
          src.location,
          format!(
            "expected `component`, `event` or `system`, but found `{}`",
            unrecognized
          ),
        ));
      }
    };

    src = parse_whitespace(src)?.src;
  }
}

#[cfg(test)]
mod tests {
  use crate::parse::{
    data::src::ParseSrc,
    function_like::Function,
    parse, strip_comments,
    struct_like::{Struct, StructField},
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
          Struct::new(
            "transform",
            vec![
              StructField::new("double", "x"),
              StructField::new("double", "y"),
            ]
          ),
          Struct::new("render", vec![StructField::new("texture_t", "texture"),]),
        ],

        events: vec![Struct::new(
          "mouse_click",
          vec![
            StructField::new("double", "x"),
            StructField::new("double", "y"),
            StructField::new("uint8_t", "button"),
          ]
        ),],

        systems: vec![
          Function::new("move", vec!["transform"]),
          Function::new("render", vec!["transform", "render"]),
        ]
      }
    );
  }
}
