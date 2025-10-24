use crate::parse::{
  basic::str::{parse_char, parse_str, parse_whitespace},
  data::{
    result::{ParseResult, ParseSuccess},
    src::ParseSrc,
  },
  expressions::{
    common::{Expression, Table},
    parse_expression,
  },
};

use super::common::TableEntry;

pub fn parse_table<'str>(
  mut src: ParseSrc<'str>,
) -> ParseResult<'str, Expression<'str>> {
  let start = src.clone();
  let mut table = Table::new();

  let marker = parse_char('{', src)?;
  src = parse_whitespace(marker.src)?.src;

  // Check if empty.
  let maybe_finish = parse_char('}', src.clone());

  if let Ok(finish) = maybe_finish {
    src = finish.src;

    return Ok(ParseSuccess {
      value: Expression::Table(table),
      span: src.span_from(&start),
      src,
    });
  }

  loop {
    let entry = parse_table_entry(src, &mut table)?;
    src = parse_whitespace(entry.src)?.src;

    if let Ok(success) = parse_char(',', src.clone()) {
      src = parse_whitespace(success.src)?.src;
    }

    let maybe_finish = parse_char('}', src.clone());

    if let Ok(finish) = maybe_finish {
      src = finish.src;
      break;
    }
  }

  Ok(ParseSuccess {
    value: Expression::Table(table),
    span: src.span_from(&start),
    src,
  })
}

fn parse_table_entry<'str>(
  mut src: ParseSrc<'str>,
  table: &mut Table<'str>,
) -> ParseResult<'str, ()> {
  let start = src.clone();

  // Check if embedding another table.
  let maybe_embed = parse_str("...", src.clone());

  if let Ok(success) = maybe_embed {
    src = success.src;
    let expr = parse_expression(src.clone())?;

    table.add_positional(TableEntry::Embed(expr.value));
    src = expr.src;

    return Ok(ParseSuccess {
      value: (),
      span: src.span_from(&start),
      src,
    });
  }

  // Not embedding.
  let expr = parse_expression(src)?;
  src = expr.src;

  // Check if keyed.
  let src = if let Expression::Symbol(name) = expr.value {
    let start = src.clone();
    let mut src = src;

    src = parse_whitespace(src)?.src;
    let maybe_eq = parse_char('=', src.clone());

    if let Ok(success) = maybe_eq {
      src = success.src;
      src = parse_whitespace(src)?.src;
      let value = parse_expression(src)?;
      src = value.src;

      table.add_keyed(name, value.value);

      return Ok(ParseSuccess {
        value: (),
        span: src.span_from(&start),
        src,
      });
    } else {
      start
    }
  } else {
    src
  };

  table.add_positional(TableEntry::Expr(expr.value));

  Ok(ParseSuccess {
    value: (),
    span: src.span_from(&start),
    src,
  })
}

#[cfg(test)]
mod tests {
  use crate::parse::{
    data::src::ParseSrc,
    expressions::{
      common::{table, Expression, Table, TableEntry},
      table::{parse_table, parse_table_entry},
    },
  };

  #[test]
  fn test_parse_table_entry() {
    // Good positional.
    let mut table = Table::new();

    let mut expected_table = Table::new();
    expected_table.add_positional(TableEntry::Expr(Expression::Symbol("foo")));

    let src = ParseSrc::new(None, "foo a");
    let result = parse_table_entry(src, &mut table).expect("parse error");
    assert_eq!(table, expected_table);
    assert_eq!(result.src.remaining_str(), " a");

    // Good embedding.
    let mut table = Table::new();

    let mut expected_table = Table::new();
    expected_table.add_positional(TableEntry::Embed(Expression::Variable("bar")));

    let src = ParseSrc::new(None, "...$bar a");
    let result = parse_table_entry(src, &mut table).expect("parse error");
    assert_eq!(table, expected_table);
    assert_eq!(result.src.remaining_str(), " a");

    // Good keyed.
    let mut table = Table::new();

    let mut expected_table = Table::new();
    expected_table.add_keyed("max", Expression::Integer(128));

    let src = ParseSrc::new(None, "max = 128 a");
    let result = parse_table_entry(src, &mut table).expect("parse error");
    assert_eq!(table, expected_table);
    assert_eq!(result.src.remaining_str(), " a");

    // Different characters.
    let src = ParseSrc::new(None, "#abc");
    let _ = parse_table(src).expect_err("parse not error");
  }

  #[test]
  fn test_parse_table() {
    // Good empty.
    let src = ParseSrc::new(None, "{} a");
    let expected_table = table!();

    let result = parse_table(src).expect("parse error");
    assert_eq!(result.value, Expression::Table(expected_table));
    assert_eq!(result.src.remaining_str(), " a");

    // Good positional symbols.
    let src = ParseSrc::new(None, "{ foo, bar, } a");
    let expected_table = table!(sym "foo", sym "bar",);

    let result = parse_table(src).expect("parse error");
    assert_eq!(result.value, Expression::Table(expected_table));
    assert_eq!(result.src.remaining_str(), " a");

    // Good keyed.
    let src = ParseSrc::new(None, "{ foo = $bar } a");
    let expected_table = table!("foo" = var "bar",);

    let result = parse_table(src).expect("parse error");
    assert_eq!(result.value, Expression::Table(expected_table));
    assert_eq!(result.src.remaining_str(), " a");

    // Good mixed.
    let src = ParseSrc::new(None, "{ 1, 2, foo = $bar, max = 50, } a");
    let expected_table = table!(int 1, int 2, "foo" = var "bar", "max" = int 50,);

    let result = parse_table(src).expect("parse error");
    assert_eq!(result.value, Expression::Table(expected_table));
    assert_eq!(result.src.remaining_str(), " a");

    // Good embed.
    let src = ParseSrc::new(None, "{ 1, 2, ...{ 3, 4 }, } a");
    let expected_table = table!(int 1, int 2, ...table!(int 3, int 4,),);

    let result = parse_table(src).expect("parse error");
    assert_eq!(result.value, Expression::Table(expected_table));
    assert_eq!(result.src.remaining_str(), " a");

    // Invalid.
    let src = ParseSrc::new(None, "abc1");
    let _ = parse_table(src).expect_err("parse not error");

    // Invalid: Lone comma.
    let src = ParseSrc::new(None, "{ , }");
    let _ = parse_table(src).expect_err("parse not error");

    // Invalid: Non-symbol key.
    let src = ParseSrc::new(None, "{ $a = b }");
    let _ = parse_table(src).expect_err("parse not error");

    // Invalid: Embed in keyed value.
    let src = ParseSrc::new(None, "{ x = ...$x }");
    let _ = parse_table(src).expect_err("parse not error");
  }
}
