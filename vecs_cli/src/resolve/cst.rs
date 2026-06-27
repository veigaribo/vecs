use std::collections::{BTreeMap, BTreeSet, HashMap};

use derive_builder::Builder;

use crate::{common::StringKind, parse::data::str::Span};

// CST means "concrete semantic tree".
// Maybe you wouldn't consider this a tree but I think `css` would be too confusing.

// Structs (components and events).

#[derive(Debug, Clone, Builder)]
pub struct TypeName<'src> {
  #[builder(field(vis = "pub"))]
  pub span: Span<'src>,

  #[builder(field(vis = "pub"))]
  pub name: &'src str,

  #[builder(field(vis = "pub"))]
  pub type_components: Vec<&'src str>,
}

impl<'src> TypeName<'src> {
  pub fn is_empty(&self) -> bool {
    self.type_components.is_empty()
  }
}

impl<'src> TypeNameBuilder<'src> {
  pub fn add_type_component(&mut self, component: &'src str) {
    if let Some(ref mut components) = self.type_components {
      components.push(component);
    } else {
      self.type_components = Some(vec![component]);
    }
  }
}

impl<'src> PartialEq for TypeName<'src> {
  fn eq(&self, other: &Self) -> bool {
    self.name == other.name
  }
}

// We make sure there are no name conflicts.
impl<'src> Eq for TypeName<'src> {}

impl<'src> std::hash::Hash for TypeName<'src> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.name.hash(state)
  }
}

// Components.

#[derive(Debug, Clone, Builder)]
pub struct Component<'src> {
  #[builder(field(vis = "pub"))]
  pub span: Span<'src>,

  // Empty => marker
  #[builder(field(vis = "pub"))]
  pub typ: TypeName<'src>,

  // mask[i] & (1 << j)
  #[builder(field(vis = "pub"))]
  pub mask_i: u16,
  #[builder(field(vis = "pub"))]
  pub mask_j: u8,
}

impl<'src> Component<'src> {
  pub fn name(&self) -> &'src str {
    self.typ.name
  }

  pub fn is_empty(&self) -> bool {
    self.typ.type_components.is_empty()
  }
}

// Nodes.

#[derive(Debug, Clone, Builder, Hash)]
pub struct Node<'src> {
  #[builder(field(vis = "pub"))]
  pub span: Span<'src>,
  #[builder(field(vis = "pub"))]
  pub name: &'src str,
  #[builder(field(vis = "pub"))]
  pub components: BTreeSet<&'src str>,

  // This vec having `n` elements does not mean the final mask will have `n` elements;
  // it just means that all components afterwards are zero.
  #[builder(default = vec![], field(vis = "pub"))]
  pub mask: Vec<u64>,
}

impl<'src> Node<'src> {
  pub fn is_empty(&self) -> bool {
    self.components.iter().all(|c| c.is_empty())
  }
}

impl<'src> PartialEq for Node<'src> {
  fn eq(&self, other: &Self) -> bool {
    self.name == other.name
  }
}

// We make sure there are no name conflicts.
impl<'src> Eq for Node<'src> {}

impl<'src> NodeBuilder<'src> {
  pub fn init_components(&mut self) {
    self.components = Some(BTreeSet::new());
  }

  pub fn add_component(&mut self, component: &'src str) {
    if let Some(ref mut components) = self.components {
      components.insert(component);
    } else {
      let mut set = BTreeSet::new();
      set.insert(component);
      self.components = Some(set);
    }
  }
}

// Systems.

#[derive(Debug, Clone, Builder)]
pub struct System<'src> {
  #[builder(field(vis = "pub"))]
  pub span: Span<'src>,
  #[builder(field(vis = "pub"))]
  pub name: &'src str,
  #[builder(field(vis = "pub"))]
  pub event: &'src str,

  // If there is no node, the system is a singleton.
  #[builder(field(vis = "pub"))]
  pub node: Option<&'src str>,

  // The amount of states this system is in. We use this to show a warning in case
  // there are zero (this system is not used at all).
  #[builder(default = 0, field(vis = "pub"))]
  pub in_state_count: usize,
}

// States.

#[derive(Debug, Clone, Builder, Hash)]
pub struct State<'src> {
  #[builder(field(vis = "pub"))]
  pub span: Span<'src>,

  #[builder(field(vis = "pub"))]
  pub name: &'src str,

  #[builder(field(vis = "pub"))]
  pub systems: Vec<Vec<&'src str>>,

  #[builder(field(vis = "pub"))]
  pub nodes: Vec<&'src str>,
}

impl<'src> PartialEq for State<'src> {
  fn eq(&self, other: &Self) -> bool {
    self.name == other.name
  }
}

// We make sure there are no name conflicts.
impl<'src> Eq for State<'src> {}

impl<'src> StateBuilder<'src> {
  pub fn add_node(&mut self, node: &'src str) {
    if let Some(ref mut nodes) = self.nodes {
      nodes.push(node);
    } else {
      self.nodes = Some(vec![node]);
    }
  }
}

// Settings.

// #[derive(Debug, Copy, Clone)]
// pub struct Settings {}

// impl Default for Settings {
//   fn default() -> Self {
//     Self {}
//   }
// }

// CST. See the top comment for what it means.

#[derive(Debug, Clone, Default)]
pub struct Cst<'src> {
  pub includes: Vec<StringKind>,
  pub globals: HashMap<&'src str, TypeName<'src>>,
  // pub settings: Settings,
  pub components: HashMap<&'src str, Component<'src>>,
  pub events: HashMap<&'src str, TypeName<'src>>,
  pub systems: HashMap<&'src str, System<'src>>,
  pub nodes: HashMap<&'src str, Node<'src>>,
  pub node_mask_arr_size: u16,

  // We use a BTreeMap instead of a HashMap to have a consistent order of iteration.
  // That is important in codegen.
  pub states: BTreeMap<&'src str, State<'src>>,
}

// These methods do not check for errors (e.g. a non-existent component in a node or
// system) because it's usually more efficient to check for such things during
// resolution.
impl<'src> Cst<'src> {
  pub fn add_include(&mut self, include: StringKind) {
    self.includes.push(include);
  }

  pub fn add_component(&mut self, typ: TypeName<'src>) {
    let index = self.components.len();

    let mask_i: u16 = (index / 64).try_into().unwrap();
    let mask_j: u8 = (index % 64).try_into().unwrap();

    let component = Component {
      span: typ.span,
      typ,
      mask_i,
      mask_j,
    };

    self.components.insert(component.name(), component);
    self.node_mask_arr_size = mask_i + 1;
  }

  pub fn add_event(&mut self, event: TypeName<'src>) {
    self.events.insert(event.name, event);
  }

  // Will panic if a component is not found.
  pub fn add_node(&mut self, mut node: Node<'src>) {
    node.mask = Vec::with_capacity(self.node_mask_arr_size.into());
    node.mask.resize(self.node_mask_arr_size.into(), 0);

    for component_name in node.components.iter() {
      let component = self.components.get(component_name).expect(&format!(
        "node contains unknown component after resolution: {}",
        component_name
      ));

      node.mask[component.mask_i as usize] |= 1 << component.mask_j;
    }

    self.nodes.insert(node.name, node);
  }

  pub fn add_system(&mut self, system: System<'src>) {
    self.systems.insert(system.name, system);
  }

  pub fn add_state(&mut self, state: State<'src>) {
    for system_names in state.systems.iter() {
      for system_name in system_names.iter() {
        let system = self.systems.get_mut(system_name);

        system
          .expect("state references non existent system")
          .in_state_count += 1;
      }
    }

    self.states.insert(state.name, state);
  }

  pub fn add_global(&mut self, global: TypeName<'src>) {
    self.globals.insert(global.name, global);
  }
}
