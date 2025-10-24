use std::collections::HashMap;

// A positional entry in a table may be a normal value or the embedment of another
// table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TableEntry<'str> {
  Expr(Expression<'str>),
  Embed(Expression<'str>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table<'str> {
  pub positional: Vec<TableEntry<'str>>,

  // Keyed fields must have a literal key and a regular value.
  pub keyed: HashMap<&'str str, Expression<'str>>,
}

impl<'str> Table<'str> {
  pub fn new() -> Self {
    Self {
      positional: Vec::new(),
      keyed: HashMap::new(),
    }
  }

  pub fn add_positional(&mut self, value: TableEntry<'str>) {
    self.positional.push(value);
  }

  pub fn add_keyed(&mut self, key: &'str str, value: Expression<'str>) {
    self.keyed.insert(key, value);
  }
}

#[cfg(test)]
macro_rules! table_internal {
  // Empty table
  ($table:ident;) => {};

  // Keyed:

  ($table:ident; $key:literal = int $value:expr, $($rest:tt)*) => {
    let key: &str = $key;
    let expr = crate::parse::expressions::common::Expression::Integer($value);

    $table.add_keyed(key, expr);
    crate::parse::expressions::common::table_internal!($table; $($rest)*);
  };

  ($table:ident; $key:literal = sym $value:expr, $($rest:tt)*) => {
    let key: &str = $key;
    let expr = crate::parse::expressions::common::Expression::Symbol($value);

    $table.add_keyed(key, expr);
    crate::parse::expressions::common::table_internal!($table; $($rest)*);
  };

  ($table:ident; $key:literal = var $value:expr, $($rest:tt)*) => {
    let key: &str = $key;
    let expr = crate::parse::expressions::common::Expression::Variable($value);

    $table.add_keyed(key, expr);
    crate::parse::expressions::common::table_internal!($table; $($rest)*);
  };

  // Positional:

  ($table:ident; ...$value:expr, $($rest:tt)*) => {
    let expr = crate::parse::expressions::common::Expression::Table($value);
    let entry = crate::parse::expressions::common::TableEntry::Embed(expr);

    $table.add_positional(entry);
    crate::parse::expressions::common::table_internal!($table; $($rest)*);
  };

  ($table:ident; int $value:expr, $($rest:tt)*) => {
    let expr = crate::parse::expressions::common::Expression::Integer($value);
    let entry = crate::parse::expressions::common::TableEntry::Expr(expr);

    $table.add_positional(entry);
    crate::parse::expressions::common::table_internal!($table; $($rest)*);
  };

  ($table:ident; sym $value:expr, $($rest:tt)*) => {
    let expr = crate::parse::expressions::common::Expression::Symbol($value);
    let entry = crate::parse::expressions::common::TableEntry::Expr(expr);

    $table.add_positional(entry);
    crate::parse::expressions::common::table_internal!($table; $($rest)*);
  };

  ($table:ident; var $value:expr, $($rest:tt)*) => {
    let expr = crate::parse::expressions::common::Expression::Variable($value);
    let entry = crate::parse::expressions::common::TableEntry::Expr(expr);

    $table.add_positional(entry);
    crate::parse::expressions::common::table_internal!($table; $($rest)*);
  };
}

#[cfg(test)]
macro_rules! table {
  () => { crate::parse::expressions::common::Table::new() };

  ($($tt:tt)*) => {{
    let mut table = crate::parse::expressions::common::Table::new();
    crate::parse::expressions::common::table_internal!(table; $($tt)*);
    table
  }};
}

#[cfg(test)]
pub(crate) use table;

#[cfg(test)]
pub(crate) use table_internal;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression<'str> {
  Integer(i128),
  Symbol(&'str str),

  Variable(&'str str),
  Table(Table<'str>),
}

#[cfg(test)]
mod tests {
  use crate::parse::expressions::common::{Expression, Table, TableEntry};

  #[test]
  fn test_table_macro() {
    let t = table!(
      int 10, sym "peter", var "pan",
      "key1" = int 20, "key2" = sym "mona", "key3" = var "lisa",
    );

    let mut expected = Table::new();
    expected.add_positional(TableEntry::Expr(Expression::Integer(10)));
    expected.add_positional(TableEntry::Expr(Expression::Symbol("peter")));
    expected.add_positional(TableEntry::Expr(Expression::Variable("pan")));
    expected.add_keyed("key1", Expression::Integer(20));
    expected.add_keyed("key2", Expression::Symbol("mona"));
    expected.add_keyed("key3", Expression::Variable("lisa"));

    assert_eq!(t, expected);
  }
}
