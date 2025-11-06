use std::fmt;

#[cfg(test)]
use crate::parse::data::str::Location;
use crate::parse::data::str::Span;

// A positional entry in a table may be a normal value or the embedment of another
// table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListEntry<'src> {
  Expr(Expression<'src>),
  Embed(Expression<'src>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressionKind<'src> {
  Integer(i128),
  Symbol(&'src str),
  Variable(&'src str),

  Application(Vec<Expression<'src>>),
  List(Vec<ListEntry<'src>>),
}

#[derive(Debug, Clone, Eq, Educe)]
#[educe(PartialEq)]
pub struct Expression<'src> {
  pub kind: ExpressionKind<'src>,

  #[educe(PartialEq(ignore))]
  pub span: Span<'src>,
}

fn write_indent<W: fmt::Write>(w: &mut W, indent: usize) -> fmt::Result {
  for _ in 0..indent {
    w.write_char(' ')?;
  }

  Ok(())
}

impl<'src> Expression<'src> {
  pub fn new(kind: ExpressionKind<'src>, span: Span<'src>) -> Self {
    Self { kind, span }
  }

  fn show<W: fmt::Write>(&self, indent: usize, w: &mut W) -> fmt::Result {
    let span = self.span;

    match &self.kind {
      ExpressionKind::Integer(value) => writeln!(w, "int ({}) {}", span, value),
      ExpressionKind::Symbol(value) => writeln!(w, "sym ({}) {}", span, value),
      ExpressionKind::Variable(value) => writeln!(w, "var ({}) {}", span, value),
      ExpressionKind::Application(expressions) => {
        if expressions.is_empty() {
          write!(w, "app ({}) {{}}\n", span)
        } else {
          write!(w, "app ({}) {{\n", span)?;

          for param in expressions {
            write_indent(w, indent)?;
            param.show(indent + 1, w)?;
          }

          write_indent(w, indent - 1)?;
          write!(w, "}}\n")
        }
      }
      ExpressionKind::List(items) => {
        if items.is_empty() {
          write!(w, "list ({}) {{}}\n", span)
        } else {
          write!(w, "list ({}) {{\n", span)?;

          for item in items {
            write_indent(w, indent)?;

            match item {
              ListEntry::Expr(expression) => {
                expression.show(indent + 1, w)?;
              }
              ListEntry::Embed(expression) => {
                write!(w, "...")?;
                expression.show(indent + 1, w)?;
              }
            }
          }

          write_indent(w, indent - 1)?;
          write!(w, "}}\n")
        }
      }
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ast<'src>(pub Vec<Expression<'src>>);

impl<'src> fmt::Display for Ast<'src> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Ast\n")?;

    for expr in self.0.iter() {
      expr.show(1, f)?;
    }

    Ok(())
  }
}

// Macros to help building expressions. Currently only for tests.

#[cfg(test)]
pub static DUMMY_SPAN: Span =
  Span::new(Location::new(Some("test")), Location::new(Some("test")));

#[cfg(test)]
macro_rules! int {
  ($value:expr) => {
    crate::parse::ast::Expression::new(
      crate::parse::ast::ExpressionKind::Integer($value),
      crate::parse::ast::DUMMY_SPAN,
    )
  };
}

use educe::Educe;
#[cfg(test)]
pub(crate) use int;

#[cfg(test)]
macro_rules! sym {
  ($value:expr) => {
    crate::parse::ast::Expression::new(
      crate::parse::ast::ExpressionKind::Symbol($value),
      crate::parse::ast::DUMMY_SPAN,
    )
  };
}

#[cfg(test)]
pub(crate) use sym;

#[cfg(test)]
macro_rules! var {
  ($value:expr) => {
    crate::parse::ast::Expression::new(
      crate::parse::ast::ExpressionKind::Variable($value),
      crate::parse::ast::DUMMY_SPAN,
    )
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
    use crate::parse::ast::{Expression, ExpressionKind, ListEntry, list_internal, DUMMY_SPAN};

    #[allow(unused_mut)]
    let mut v = Vec::<ListEntry>::new();
    list_internal!(v; $($tt)*);

    Expression::new(
      ExpressionKind::List(v),
      DUMMY_SPAN,
    )
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
    use crate::parse::ast::{Expression, ExpressionKind, app_internal, DUMMY_SPAN};

    #[allow(unused_mut)]
    let mut v = Vec::<Expression>::new();
    app_internal!(v; $($tt)*);

    Expression::new(
      ExpressionKind::Application(v),
      DUMMY_SPAN,
    )
  }};
}

#[cfg(test)]
pub(crate) use app;
