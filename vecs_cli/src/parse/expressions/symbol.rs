use crate::parse::{
  ast::Expression,
  data::{result::ParseResult, src::ParseSrc},
  util::identifiers::parse_identifier,
};

pub fn parse_symbol<'str>(
  src: ParseSrc<'str>,
) -> ParseResult<'str, Expression<'str>> {
  parse_identifier(src).map(|success| success.map(|name| Expression::Symbol(name)))
}

#[cfg(test)]
mod tests {
  use crate::parse::{
    ast::sym, data::src::ParseSrc, expressions::symbol::parse_symbol,
  };

  #[test]
  fn test_parse_symbol() {
    // Good EOF.
    let src = ParseSrc::new(None, "vec2");
    let result = parse_symbol(src).expect("parse error");
    assert_eq!(result.value, sym!("vec2"));
    assert_eq!(result.src.remaining_str(), "");

    // Good.
    let src = ParseSrc::new(None, "vec3 vec2");
    let result = parse_symbol(src).expect("parse error");
    assert_eq!(result.value, sym!("vec3"));
    assert_eq!(result.src.remaining_str(), " vec2");

    // Different characters.
    let src = ParseSrc::new(None, "1abc");
    let _ = parse_symbol(src).expect_err("parse not error");
  }
}
