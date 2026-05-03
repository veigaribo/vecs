use std::fmt::Display;

pub struct EventStructName<'a> {
  pub name: &'a str,
}

impl<'a> Display for EventStructName<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "vecs_event_{}", self.name)
  }
}

pub struct ComponentStructName<'a> {
  pub name: &'a str,
}

impl<'a> Display for ComponentStructName<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "vecs_component_{}", self.name)
  }
}

pub struct StateStructName<'a> {
  pub name: &'a str,
}

impl<'a> Display for StateStructName<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "vecs_state_{}", self.name)
  }
}

pub struct NodeStructName<'a> {
  pub name: &'a str,
}

impl<'a> Display for NodeStructName<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "vecs_node_{}", self.name)
  }
}
