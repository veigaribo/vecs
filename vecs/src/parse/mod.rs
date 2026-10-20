pub mod ast;
pub mod comments;
pub mod data;
pub mod expressions;
mod util;

use std::collections::VecDeque;

use crate::parse::{
  ast::{Ast, Expression},
  comments::parse_comment,
  data::{
    result::{ParseResult, ParseSuccess},
    src::ParseSrc,
  },
  expressions::parse_expression,
  util::str::{parse_char, parse_whitespace},
};

pub fn strip_comments(t: &mut str) {
  let mut src = ParseSrc::from(&*t);
  let mut spans = Vec::<(usize, usize)>::new(); // Just start and end bytes.

  loop {
    match parse_comment(src.clone()) {
      Ok(success) => {
        src = success.src;

        let span = success.span;
        spans.push((span.start_byte_offset, span.end_byte_offset));
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

pub fn parse<'src>(mut src: ParseSrc<'src>) -> ParseResult<'src, Ast<'src>> {
  let start = src.clone();
  let mut parsed = VecDeque::<Expression>::new();

  src = parse_whitespace(src)?.src;

  let whatever =
    parse_char(';', src.clone()).or_else(|_| parse_char(',', src.clone()));

  if let Ok(success) = whatever {
    src = parse_whitespace(success.src)?.src;
  }

  while !src.is_empty() {
    let expr = parse_expression(src)?;
    src = parse_whitespace(expr.src)?.src;
    parsed.push_back(expr.value);

    let mut whatever =
      parse_char(';', src.clone()).or_else(|_| parse_char(',', src.clone()));

    // Gobble extraneous separators.
    while let Ok(success) = whatever {
      src = parse_whitespace(success.src)?.src;
      whatever =
        parse_char(';', src.clone()).or_else(|_| parse_char(',', src.clone()));
    }
  }

  return Ok(ParseSuccess {
    value: Ast(parsed),
    span: src.span_from(&start),
    src,
  });
}

#[cfg(test)]
mod tests {
  use std::collections::VecDeque;

  use crate::parse::{
    ast::{Ast, app, list, sym},
    data::src::ParseSrc,
    parse, strip_comments,
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
      component transform {\n\
        double x;\n\
        double y;\n\
      };\n\
      \n\
      component render {\n\
        texture_t texture;\n\
      };\n\
      \n\
      event mouse_click {\n\
        double x;\n\
        double y;\n\
        uint8_t button;\n\
      };\n\
      \n\
      system move { transform };\n\
      system render { transform, render };\n\
      ",
    );

    let result = parse(src).expect("parse error");
    assert_eq!(
      result.value,
      Ast(VecDeque::from([
        app!(
          sym!("component"),
          sym!("transform"),
          list!(
            app!(sym!("double"), sym!("x")),
            app!(sym!("double"), sym!("y")),
          )
        ),
        app!(
          sym!("component"),
          sym!("render"),
          list!(app!(sym!("texture_t"), sym!("texture"))),
        ),
        app!(
          sym!("event"),
          sym!("mouse_click"),
          list!(
            app!(sym!("double"), sym!("x")),
            app!(sym!("double"), sym!("y")),
            app!(sym!("uint8_t"), sym!("button")),
          ),
        ),
        app!(sym!("system"), sym!("move"), list!(app!(sym!("transform"))),),
        app!(
          sym!("system"),
          sym!("render"),
          list!(app!(sym!("transform")), app!(sym!("render"))),
        ),
      ]))
    );
  }
}
