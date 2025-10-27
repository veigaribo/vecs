// A positional entry in a table may be a normal value or the embedment of another
// table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListEntry<'str> {
  Expr(Expression<'str>),
  Embed(Expression<'str>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression<'str> {
  Integer(i128),
  Symbol(&'str str),
  Variable(&'str str),

  Application(Vec<Expression<'str>>),
  List(Vec<ListEntry<'str>>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ast<'str>(pub Vec<Expression<'str>>);

// Macros to help building expressions. Currently only for tests.

#[cfg(test)]
macro_rules! int {
  ($value:expr) => {
    crate::parse::ast::Expression::Integer($value)
  };
}

#[cfg(test)]
pub(crate) use int;

#[cfg(test)]
macro_rules! sym {
  ($value:expr) => {
    crate::parse::ast::Expression::Symbol($value)
  };
}

#[cfg(test)]
pub(crate) use sym;

#[cfg(test)]
macro_rules! var {
  ($value:expr) => {
    crate::parse::ast::Expression::Variable($value)
  };
}

#[cfg(test)]
pub(crate) use var;

#[cfg(test)]
macro_rules! list_internal {
  ($list:ident;) => {};

  ($list:ident; ...$value:expr, $($rest:tt)*) => {
    let entry = crate::parse::ast::ListEntry::Embed($value);

    $list.push(entry);
    crate::parse::ast::list_internal!($list; $($rest)*);
  };

  ($list:ident; ...$value:expr) => {
    let entry = crate::parse::ast::ListEntry::Embed($value);

    $list.push(entry);
  };

  ($list:ident; $value:expr, $($rest:tt)*) => {
    let entry = crate::parse::ast::ListEntry::Expr($value);

    $list.push(entry);
    crate::parse::ast::list_internal!($list; $($rest)*);
  };

  ($list:ident; $value:expr) => {
    let entry = crate::parse::ast::ListEntry::Expr($value);

    $list.push(entry);
  };
}

#[cfg(test)]
pub(crate) use list_internal;

#[cfg(test)]
macro_rules! list {
  ($($tt:tt)*) => {{
    use crate::parse::ast::{Expression, ListEntry, list_internal};

    #[allow(unused_mut)]
    let mut v = Vec::<ListEntry>::new();
    list_internal!(v; $($tt)*);
    Expression::List(v)
  }};
}

#[cfg(test)]
pub(crate) use list;

#[cfg(test)]
macro_rules! app_internal {
  ($seq:ident;) => {};

  ($seq:ident; $value:expr, $($rest:tt)*) => {
    $seq.push($value);
    crate::parse::ast::app_internal!($seq; $($rest)*);
  };

  ($seq:ident; $value:expr) => {
    $seq.push($value);
  };
}

#[cfg(test)]
pub(crate) use app_internal;

#[cfg(test)]
macro_rules! app {
  ($($tt:tt)*) => {{
    use crate::parse::ast::{Expression, app_internal};

    #[allow(unused_mut)]
    let mut v = Vec::<Expression>::new();
    app_internal!(v; $($tt)*);
    Expression::Application(v)
  }};
}

#[cfg(test)]
pub(crate) use app;
