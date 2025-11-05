use std::collections::HashMap;

use derive_builder::Builder;

// CST means "concrete semantic tree".
// Maybe you wouldn't consider this a tree but I think `css` would be too confusing.

// Structs (components and events).

#[derive(Debug, Clone, Builder)]
pub struct StructField<'a> {
  // Vec of identifier components (like vec![unsigned, long, long, int]).
  pub typ: Vec<&'a str>,
  pub name: &'a str,
}

#[derive(Debug, Clone, Builder)]
pub struct Struct<'a> {
  pub name: &'a str,
  pub fields: Vec<StructField<'a>>,
}

impl<'a> StructBuilder<'a> {
  pub fn add_field(&mut self, field: StructField<'a>) {
    if let Some(ref mut fields) = self.fields {
      fields.push(field);
    } else {
      self.fields = Some(vec![field]);
    }
  }
}

// Systems.

#[derive(Debug, Clone, Builder)]
pub struct System<'a> {
  pub name: &'a str,
  pub params: Vec<&'a str>,
}

impl<'a> SystemBuilder<'a> {
  pub fn add_param(&mut self, param: &'a str) {
    if let Some(ref mut params) = self.params {
      params.push(param);
    } else {
      self.params = Some(vec![param]);
    }
  }
}

// States.

#[derive(Debug, Clone, Builder)]
pub struct StateComponent<'a> {
  pub name: &'a str,
  pub max: Option<u64>,
}

#[derive(Debug, Clone, Builder)]
pub struct State<'a> {
  pub name: &'a str,

  #[builder(field(vis = "pub"))]
  pub components: Vec<StateComponent<'a>>,

  #[builder(field(vis = "pub"))]
  pub systems: Vec<Vec<&'a str>>,
}

// CST. See the top comment for what it means.

#[derive(Debug, Clone, Default)]
pub struct Cst<'a> {
  pub components: HashMap<&'a str, Struct<'a>>,
  pub events: HashMap<&'a str, Struct<'a>>,
  pub systems: HashMap<&'a str, System<'a>>,

  pub states: Vec<State<'a>>,
}

impl<'a> Cst<'a> {
  pub fn add_component(&mut self, component: Struct<'a>) {
    self.components.insert(component.name, component);
  }

  pub fn add_event(&mut self, event: Struct<'a>) {
    self.events.insert(event.name, event);
  }

  pub fn add_system(&mut self, system: System<'a>) {
    self.systems.insert(system.name, system);
  }

  pub fn add_state(&mut self, state: State<'a>) {
    self.states.push(state);
  }
}
