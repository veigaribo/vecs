use crate::parse::{
  basic::{
    identifiers::{is_identifier, parse_identifier},
    str::{parse_char, parse_whitespace},
  },
  data::{
    result::{ParseResult, ParseSuccess},
    src::ParseSrc,
  },
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct StructField<'str> {
  pub typ: &'str str,
  pub name: &'str str,
}

impl<'str> StructField<'str> {
  pub fn new(typ: &'str str, name: &'str str) -> Self {
    debug_assert!(is_identifier(name));

    Self { typ, name }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Struct<'str> {
  pub name: &'str str,
  pub fields: Vec<StructField<'str>>,
}

impl<'str> Struct<'str> {
  pub fn new(name: &'str str, fields: Vec<StructField<'str>>) -> Self {
    Self { name, fields }
  }
}

pub fn parse_struct<'str>(
  mut src: ParseSrc<'str>,
) -> ParseResult<'str, Struct<'str>> {
  let start = src.clone();
  let mut fields = Vec::<StructField>::new();

  let parsed_name = parse_identifier(src)?;
  src = parsed_name.src;

  src = parse_whitespace(src)?.src;
  src = parse_char('{', src)?.src;
  src = parse_whitespace(src)?.src;

  loop {
    let maybe_finish = parse_char('}', src.clone());

    if let Ok(finish) = maybe_finish {
      src = finish.src;
      break;
    }

    let field = parse_struct_field(src)?;
    fields.push(field.value);
    src = parse_whitespace(field.src)?.src;
  }

  Ok(ParseSuccess {
    value: Struct::new(parsed_name.value, fields),
    span: src.span_from(&start),
    src,
  })
}

pub fn parse_struct_field<'str>(
  mut src: ParseSrc<'str>,
) -> ParseResult<'str, StructField<'str>> {
  let start = src.clone();

  let mut parsed_type: ParseSuccess<'str, &'str str>;
  let parsed_name: ParseSuccess<'str, &'str str>;

  parsed_type = parse_identifier(src)?;
  src = parsed_type.src;

  src = parse_whitespace(src)?.src;

  // Whatever is last is the name.
  loop {
    let parsed_cont = parse_identifier(src)?;
    src = parsed_cont.src.clone();
    src = parse_whitespace(src)?.src;

    let maybe_finish = parse_char(';', src.clone());

    if let Ok(finish) = maybe_finish {
      src = finish.src;
      parsed_name = parsed_cont;
      break;
    }

    let span = src.span_from(&start);

    parsed_type = ParseSuccess {
      value: &src.slice(span).trim(),
      span,
      src: src.clone(),
    };
  }

  Ok(ParseSuccess {
    value: StructField::new(parsed_type.value, parsed_name.value),
    span: src.span_from(&start),
    src,
  })
}

#[cfg(test)]
mod tests {
  use crate::parse::{
    data::src::ParseSrc,
    struct_like::{parse_struct, parse_struct_field, Struct, StructField},
  };

  #[test]
  fn test_parse_struct_field() {
    // Good.
    let src = ParseSrc::new(None, "int x; // a");
    let result = parse_struct_field(src).expect("parse error");
    assert_eq!(result.value, StructField::new("int", "x"));
    assert_eq!(result.src.remaining_str(), " // a");

    // Different characters.
    let src = ParseSrc::new(None, ";abc");
    let _ = parse_struct_field(src).expect_err("parse not error");
  }

  #[test]
  fn test_parse_struct() {
    // Good.
    let src = ParseSrc::new(
      None,
      "vec2 {\
      double x;\
      double y;\
      struct metadata meta;\
    } // math",
    );

    let result = parse_struct(src).expect("parse error");
    assert_eq!(
      result.value,
      Struct::new(
        "vec2",
        vec![
          StructField::new("double", "x"),
          StructField::new("double", "y"),
          StructField::new("struct metadata", "meta")
        ]
      )
    );
    assert_eq!(result.src.remaining_str(), " // math");

    // Good. No fields.
    let src = ParseSrc::new(
      None,
      "vec0 {\
    } // a",
    );

    let result = parse_struct(src).expect("parse error");
    assert_eq!(result.value, Struct::new("vec0", vec![]));
    assert_eq!(result.src.remaining_str(), " // a");

    // Different characters.
    let src = ParseSrc::new(None, ";abc");
    let _ = parse_struct(src).expect_err("parse not error");
  }
}
