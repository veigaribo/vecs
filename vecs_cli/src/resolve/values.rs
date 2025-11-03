use std::{collections::HashMap, fmt::Display};

use educe::Educe;

use crate::{
  parse::{
    ast::{Expression, ExpressionKind, ListEntry},
    data::str::Span,
  },
  resolve::result::{ResolveError, ResolveResult},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueKind<'a> {
  Integer(i128),
  Symbol(&'a str),
  List(Vec<Value<'a>>),
}

#[derive(Debug, Clone, Eq, Educe)]
#[educe(PartialEq)]
pub struct Value<'a> {
  pub kind: ValueKind<'a>,

  #[educe(PartialEq(ignore))]
  pub span: Span<'a>,
}

impl<'a> Value<'a> {
  pub fn new(kind: ValueKind<'a>, span: Span<'a>) -> Self {
    Value { kind, span }
  }
}

impl<'a> Display for Value<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self.kind {
      ValueKind::Integer(x) => write!(f, "{}", x),
      ValueKind::Symbol(name) => write!(f, "`{}`", name),
      ValueKind::List(ref values) => {
        if values.is_empty() {
          write!(f, "{{}}")
        } else {
          write!(f, "{{ ")?;

          let head = &values[0];
          head.fmt(f)?;

          for value in &values[1..] {
            write!(f, ", {}", value)?;
          }

          write!(f, " }}")
        }
      }
    }
  }
}

#[derive(Debug, Clone)]
pub struct VarTable<'a> {
  pub variables: HashMap<&'a str, Value<'a>>,
}

impl<'a> VarTable<'a> {
  pub fn new() -> Self {
    Self {
      variables: HashMap::new(),
    }
  }

  // pub fn add(&mut self, name: &'a str, value: Value<'a>) {
  //   self.variables.insert(name, value);
  // }

  pub fn resolve_var(&self, name: &'a str) -> Option<Value<'a>> {
    self.variables.get(name).cloned()
  }

  pub fn resolve(&self, expr: &Expression<'a>) -> ResolveResult<'a, Value<'a>> {
    let span = expr.span;

    match &expr.kind {
      ExpressionKind::Integer(x) => Ok(Value::new(ValueKind::Integer(*x), span)),
      ExpressionKind::Symbol(x) => Ok(Value::new(ValueKind::Symbol(*x), span)),
      ExpressionKind::Variable(name) => match self.resolve_var(*name) {
        Some(value) => Ok(value),
        None => Err(ResolveError::new(
          span,
          format!("could not resolve variable ${}", name),
        )),
      },
      ExpressionKind::Application(items) => {
        let mut resolved = Vec::<Value>::with_capacity(items.len());

        for entry in items {
          let value = self.resolve(entry)?;
          resolved.push(value);
        }

        Ok(Value::new(ValueKind::List(resolved), span))
      }
      ExpressionKind::List(items) => {
        let mut resolved = Vec::<Value>::with_capacity(items.len());

        for entry in items {
          match entry {
            ListEntry::Expr(expression) => {
              let value = self.resolve(expression)?;
              resolved.push(value);
            }
            ListEntry::Embed(expression) => {
              let value = self.resolve(expression)?;

              if let ValueKind::List(mut inner) = value.kind {
                resolved.append(&mut inner);
              } else {
                return Err(ResolveError::new(
                  span,
                  format!("tried to embed non-list value: {}", value),
                ));
              }
            }
          }
        }

        Ok(Value::new(ValueKind::List(resolved), span))
      }
    }
  }
}
