use std::{
  collections::{HashMap, VecDeque},
  fmt::Display,
};

use educe::Educe;

use crate::{
  common::StringKind,
  parse::{
    ast::{Expression, ExpressionKind, ListEntry},
    data::str::Span,
  },
  resolve::result::{ResolveError, ResolveResult},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueKind<'src> {
  Integer(i128),
  String(StringKind),
  Symbol(&'src str),
  List(VecDeque<Value<'src>>),
  Application(VecDeque<Value<'src>>),
}

#[derive(Debug, Clone, Eq, Educe)]
#[educe(PartialEq)]
pub struct Value<'src> {
  pub kind: ValueKind<'src>,

  #[educe(PartialEq(ignore))]
  pub span: Span<'src>,
}

impl<'src> Value<'src> {
  pub fn new(kind: ValueKind<'src>, span: Span<'src>) -> Self {
    Value { kind, span }
  }
}

impl<'src> Display for Value<'src> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self.kind {
      ValueKind::Integer(x) => write!(f, "{}", x),
      ValueKind::String(ref x) => x.fmt(f),
      ValueKind::Symbol(name) => write!(f, "`{}`", name),
      ValueKind::List(ref values) => {
        if values.is_empty() {
          write!(f, "{{}}")
        } else {
          write!(f, "{{ ")?;

          let mut iter = values.iter();
          let head = iter.next().unwrap();
          head.fmt(f)?;

          for value in iter {
            write!(f, ", {}", value)?;
          }

          write!(f, " }}")
        }
      }
      ValueKind::Application(ref values) => {
        if values.is_empty() {
          write!(f, "()")
        } else {
          write!(f, "( ")?;

          let mut iter = values.iter();
          let head = iter.next().unwrap();
          head.fmt(f)?;

          for value in iter {
            write!(f, ", {}", value)?;
          }

          write!(f, " )")
        }
      }
    }
  }
}

#[derive(Debug, Clone)]
pub struct VarTable<'src> {
  pub variables: HashMap<&'src str, Value<'src>>,
}

impl<'src> VarTable<'src> {
  pub fn new() -> Self {
    Self {
      variables: HashMap::new(),
    }
  }

  // pub fn add(&mut self, name: &'src str, value: Value<'src>) {
  //   self.variables.insert(name, value);
  // }

  pub fn resolve_var(&self, name: &'src str) -> Option<Value<'src>> {
    self.variables.get(name).cloned()
  }

  pub fn resolve(&self, expr: Expression<'src>) -> ResolveResult<'src, Value<'src>> {
    let span = expr.span;

    match expr.kind {
      ExpressionKind::Integer(x) => Ok(Value::new(ValueKind::Integer(x), span)),
      ExpressionKind::String(x) => Ok(Value::new(ValueKind::String(x.into()), span)),
      ExpressionKind::Symbol(x) => Ok(Value::new(ValueKind::Symbol(x), span)),
      ExpressionKind::Variable(name) => match self.resolve_var(name) {
        Some(value) => Ok(value),
        None => Err(ResolveError::new(
          span,
          format!("could not resolve variable ${}", name),
        )),
      },
      ExpressionKind::Application(items) => {
        let mut resolved = VecDeque::<Value>::with_capacity(items.len());

        for entry in items {
          let value = self.resolve(entry)?;
          resolved.push_back(value);
        }

        Ok(Value::new(ValueKind::Application(resolved), span))
      }
      ExpressionKind::List(items) => {
        let mut resolved = VecDeque::<Value>::with_capacity(items.len());

        for entry in items {
          match entry {
            ListEntry::Expr(expression) => {
              let value = self.resolve(expression)?;
              resolved.push_back(value);
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
