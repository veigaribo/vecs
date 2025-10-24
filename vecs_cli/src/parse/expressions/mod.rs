pub mod common;
pub mod integer;
pub mod symbol;
pub mod table;
pub mod variable;

use crate::parse::{
  data::{result::ParseResult, src::ParseSrc},
  expressions::{
    common::Expression, integer::parse_integer, symbol::parse_symbol,
    table::parse_table, variable::parse_variable,
  },
};

pub fn parse_expression<'str>(
  src: ParseSrc<'str>,
) -> ParseResult<'str, Expression<'str>> {
  parse_variable(src.clone()).or_else(|_| {
    parse_integer(src.clone())
      .or_else(|_| parse_table(src.clone()).or_else(|_| parse_symbol(src)))
  })
}
