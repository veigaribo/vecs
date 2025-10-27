pub mod application;
pub mod integer;
pub mod list;
pub mod symbol;
pub mod variable;

use crate::parse::{
  ast::Expression,
  data::{
    result::{ParseError, ParseResult},
    src::ParseSrc,
  },
  expressions::{
    application::parse_application, integer::parse_integer, list::parse_list,
    symbol::parse_symbol, variable::parse_variable,
  },
};

fn parse_basic_expression<'str>(
  src: ParseSrc<'str>,
) -> ParseResult<'str, Expression<'str>> {
  let head = src.peek();

  if head.is_none() {
    return Err(ParseError::new(
      src.location,
      "expected a basic expression, but the source ended",
    ));
  }

  // Use knowledge about specific expressions to provide better error messages.
  match head.unwrap() {
    '$' => parse_variable(src.clone()),
    '{' => parse_list(src.clone()),
    _ => parse_integer(src.clone())
      .or_else(|_| parse_symbol(src.clone()))
      .map_err(|err| {
        let next = src.peek_next_token_ish();
        err.sub_message(format!("unrecognized expression: `{}`", &next))
      }),
  }
}

pub fn parse_expression<'str>(
  src: ParseSrc<'str>,
) -> ParseResult<'str, Expression<'str>> {
  parse_application(src)
}
