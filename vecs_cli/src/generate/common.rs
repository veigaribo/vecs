use std::fmt::Display;

use derive_display_hash::DisplayHash;

#[derive(Debug, Clone, DisplayHash)]
pub struct EventStructName<'a> {
  pub event_name: &'a str,
}

impl<'a> EventStructName<'a> {
  pub fn new(name: &'a str) -> Self {
    Self { event_name: name }
  }
}

impl<'a> Display for EventStructName<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "vecs_event_{}_t", self.event_name)
  }
}

#[derive(Debug, Clone, DisplayHash)]
pub struct ComponentStructName<'a> {
  pub component_name: &'a str,
}

impl<'a> ComponentStructName<'a> {
  pub fn new(name: &'a str) -> Self {
    Self {
      component_name: name,
    }
  }
}

impl<'a> Display for ComponentStructName<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "vecs_component_{}_t", self.component_name)
  }
}

#[derive(Debug, Clone, DisplayHash)]
pub struct StateStructName<'a> {
  pub state_name: &'a str,
}

impl<'a> StateStructName<'a> {
  pub fn new(name: &'a str) -> Self {
    Self { state_name: name }
  }
}

impl<'a> Display for StateStructName<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "vecs_state_{}_t", self.state_name)
  }
}

#[derive(Debug, Clone, DisplayHash)]
pub struct NodeStructName<'a> {
  pub node_name: &'a str,
}

impl<'a> NodeStructName<'a> {
  pub fn new(name: &'a str) -> Self {
    Self { node_name: name }
  }
}

impl<'a> Display for NodeStructName<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "vecs_node_{}_t", self.node_name)
  }
}
