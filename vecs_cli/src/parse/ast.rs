use crate::parse::{expressions::common::Expression, struct_def_like::Struct};

// TODO: Include source location information.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Component<'str> {
  pub name: &'str str,
  pub content: Struct<'str>,
}

impl<'str> Component<'str> {
  pub fn new(name: &'str str, content: Struct<'str>) -> Self {
    Self { name, content }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event<'str> {
  pub name: &'str str,
  pub content: Struct<'str>,
}

impl<'str> Event<'str> {
  pub fn new(name: &'str str, content: Struct<'str>) -> Self {
    Self { name, content }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct System<'str> {
  pub name: &'str str,
  pub content: Expression<'str>,
}

impl<'str> System<'str> {
  pub fn new(name: &'str str, content: Expression<'str>) -> Self {
    Self { name, content }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ast<'str> {
  pub components: Vec<Component<'str>>,
  pub events: Vec<Event<'str>>,
  pub systems: Vec<System<'str>>,
}

impl<'str> Ast<'str> {
  pub fn new() -> Self {
    Self {
      components: Vec::new(),
      events: Vec::new(),
      systems: Vec::new(),
    }
  }
}
