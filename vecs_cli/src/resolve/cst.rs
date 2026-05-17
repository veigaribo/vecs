use std::collections::{BTreeSet, HashMap};

use derive_builder::Builder;

use crate::parse::data::str::Span;

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
  pub span: Span<'src>,

  pub name: &'src str,
  pub fields: Vec<StructField<'src>>,
}

impl<'src> PartialEq for Struct<'src> {
  fn eq(&self, other: &Self) -> bool {
    self.name == other.name
  }
}

// We make sure there are no name conflicts.
impl<'src> Eq for Struct<'src> {}

impl<'src> std::hash::Hash for Struct<'src> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.name.hash(state)
  }
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

// Components.

#[derive(Debug, Clone, Builder)]
pub struct Component<'src> {
  pub span: Span<'src>,

  pub strukt: Struct<'src>,
  // mask[i] & (1 << j)
  pub mask_i: u16,
  pub mask_j: u8,
}

impl<'src> Component<'src> {
  pub fn name(&self) -> &'src str {
    self.strukt.name
  }

  // pub fn id(&self) -> u16 {
  //   self.mask_i << 6 & (self.mask_j as u16)
  // }
}

// Nodes.

#[derive(Debug, Clone, Builder, PartialEq, Eq, Hash)]
pub struct Node<'src> {
  pub span: Span<'src>,

  pub name: &'src str,
  pub components: BTreeSet<&'src str>,

  // This vec having `n` elements does not mean the final mask will have `n` elements;
  // it just means that all components afterwards are zero.
  #[builder(default = vec![])]
  pub mask: Vec<u64>,
}

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
  pub span: Span<'src>,

  pub name: &'src str,
  pub event: &'src str,
  pub node: &'src str,
}

// States.

#[derive(Debug, Clone, Builder)]
pub struct State<'src> {
  pub span: Span<'src>,

  pub name: &'src str,

  #[builder(field(vis = "pub"))]
  pub systems: Vec<Vec<&'src str>>,

  #[builder(field(vis = "pub"))]
  pub nodes: Vec<&'src str>,
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
  // pub settings: Settings,
  pub components: HashMap<&'src str, Component<'src>>,
  pub events: HashMap<&'src str, Struct<'src>>,
  pub systems: HashMap<&'src str, System<'src>>,
  pub nodes: HashMap<&'src str, Node<'src>>,
  pub node_mask_arr_size: u16,

  pub states: HashMap<&'src str, State<'src>>,
}

// These methods do not check for errors (e.g. a non-existant component in a node or
// system) because it's usually more efficient to check for such things in the middle
// of resolution.
impl<'src> Cst<'src> {
  pub fn add_component(&mut self, strukt: Struct<'src>) {
    let name = strukt.name;
    let index = self.components.len();

    // We want to be able to put `i` and `j` together as a 16-bit integer. `j` will
    // always be a number between [0, 63], so it fits nicely in 6 bits. That leaves
    // us 16-6 bits for `i`.
    let max_components = 1 << (16 - 6);
    if index > max_components {
      panic!(
        "too many components. currently the maximum supported is {}",
        max_components
      );
    }

    let mask_i: u16 = (index / 64).try_into().unwrap();
    let mask_j: u8 = (index % 64).try_into().unwrap();

    let component = Component {
      span: strukt.span,
      strukt,
      mask_i,
      mask_j,
    };

    self.components.insert(name, component);
    self.node_mask_arr_size = mask_i + 1;
  }

  pub fn add_event(&mut self, event: Struct<'src>) {
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
    self.states.insert(state.name, state);
  }
}
