use std::collections::HashMap;

use derive_builder::Builder;

// CST means "concrete semantic tree".
// Maybe you wouldn't consider this a tree but I think `css` would be too confusing.

// Structs (components and events).

#[derive(Debug, Clone, Builder)]
pub struct StructField<'src> {
  // Vec of identifier components (like vec![unsigned, long, long, int]).
  pub typ: Vec<&'src str>,
  pub name: &'src str,
}

#[derive(Debug, Clone, Builder)]
pub struct Struct<'src> {
  pub name: &'src str,
  pub fields: Vec<StructField<'src>>,
}

impl<'src> StructBuilder<'src> {
  pub fn add_field(&mut self, field: StructField<'src>) {
    if let Some(ref mut fields) = self.fields {
      fields.push(field);
    } else {
      self.fields = Some(vec![field]);
    }
  }
}

// Systems.

#[derive(Debug, Clone, Builder)]
pub struct System<'src> {
  pub name: &'src str,
  pub params: Vec<&'src str>,
}

impl<'src> SystemBuilder<'src> {
  pub fn add_param(&mut self, param: &'src str) {
    if let Some(ref mut params) = self.params {
      params.push(param);
    } else {
      self.params = Some(vec![param]);
    }
  }
}

// States.

#[derive(Debug, Clone, Builder)]
pub struct StateComponent<'src> {
  pub name: &'src str,
  pub max: Option<u64>,
}

#[derive(Debug, Clone, Builder)]
pub struct State<'src> {
  pub name: &'src str,

  #[builder(field(vis = "pub"))]
  pub components: Vec<StateComponent<'src>>,

  #[builder(field(vis = "pub"))]
  pub systems: Vec<Vec<&'src str>>,
}

// Settings.

#[derive(Debug, Copy, Clone)]
pub struct Settings {
  pub default_component_max: usize,
}

impl Default for Settings {
  fn default() -> Self {
    Self {
      default_component_max: 200,
    }
  }
}

// CST. See the top comment for what it means.

#[derive(Debug, Clone, Default)]
pub struct Cst<'src> {
  pub settings: Settings,
  pub components: HashMap<&'src str, Struct<'src>>,
  pub events: HashMap<&'src str, Struct<'src>>,
  pub systems: HashMap<&'src str, System<'src>>,

  pub states: Vec<State<'src>>,
}

impl<'src> Cst<'src> {
  pub fn add_component(&mut self, component: Struct<'src>) {
    self.components.insert(component.name, component);
  }

  pub fn add_event(&mut self, event: Struct<'src>) {
    self.events.insert(event.name, event);
  }

  pub fn add_system(&mut self, system: System<'src>) {
    self.systems.insert(system.name, system);
  }

  pub fn add_state(&mut self, state: State<'src>) {
    self.states.push(state);
  }
}
