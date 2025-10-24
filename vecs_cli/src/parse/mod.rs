pub mod ast;
pub mod basic;
pub mod comments;
pub mod data;
pub mod expressions;
pub mod struct_def_like;

use crate::parse::{
  ast::{Ast, Component, Event, System},
  basic::{identifiers::parse_identifier, str::parse_whitespace},
  comments::parse_comment,
  data::{
    result::{ParseError, ParseResult, ParseSuccess},
    src::ParseSrc,
  },
  expressions::parse_expression,
  struct_def_like::parse_struct_def,
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

pub fn parse<'str>(mut src: ParseSrc<'str>) -> ParseResult<'str, Ast<'str>> {
  let start = src.clone();
  let mut parsed = Ast::new();

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
        let success = parse_struct_def(src.clone())?;
        let name = success.value.name;

        parsed.components.push(Component::new(name, success.value));
        src = success.src;
      }
      "event" => {
        let success = parse_struct_def(src.clone())?;
        let name = success.value.name;

        parsed.events.push(Event::new(name, success.value));
        src = success.src;
      }
      "system" => {
        let name = parse_identifier(src)?;
        src = parse_whitespace(name.src)?.src;

        let expr = parse_expression(src.clone())?;
        src = expr.src;

        parsed.systems.push(System::new(name.value, expr.value));
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
    ast::{Ast, Component, Event, System},
    data::src::ParseSrc,
    expressions::common::{table, Expression},
    parse, strip_comments,
    struct_def_like::{Struct, StructField},
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
      system move { transform } \
      system render { transform, render } \
      ",
    );

    let result = parse(src).expect("parse error");
    assert_eq!(
      result.value,
      Ast {
        components: vec![
          Component::new(
            "transform",
            Struct::new(
              "transform",
              vec![
                StructField::new("double", "x"),
                StructField::new("double", "y"),
              ]
            )
          ),
          Component::new(
            "render",
            Struct::new("render", vec![StructField::new("texture_t", "texture"),])
          ),
        ],

        events: vec![Event::new(
          "mouse_click",
          Struct::new(
            "mouse_click",
            vec![
              StructField::new("double", "x"),
              StructField::new("double", "y"),
              StructField::new("uint8_t", "button"),
            ]
          )
        ),],

        systems: vec![
          System::new("move", Expression::Table(table!(sym "transform",))),
          System::new(
            "render",
            Expression::Table(table!(sym "transform", sym "render",))
          ),
        ]
      }
    );
  }
}
