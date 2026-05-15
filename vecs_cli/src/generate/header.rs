use std::fmt::Display;

use crate::resolve::cst::{Component, Cst, Node};

use super::{
  common::{ComponentStructName, EventStructName, NodeStructName, StateStructName},
  generics::{
    dyn_arrays::DynArray, dyn_queue::DynQueue, hash_dyn_arrays::HashDynArray,
    sparse_dyn_arrays::SparseDynArray,
  },
};

pub struct Header<'a> {
  pub data: &'a Cst<'a>,
}

// Formats masks of components.
pub struct ComponentMask {
  mask_size: u16,
  mask_i: u16,
  mask_j: u8,
}

impl ComponentMask {
  pub fn from_component(c: &Component, mask_size: u16) -> Self {
    Self {
      mask_size,
      mask_i: c.mask_i,
      mask_j: c.mask_j,
    }
  }
}

impl Display for ComponentMask {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.mask_size == 0 {
      return "{}".fmt(f);
    }

    write!(f, "{{")?;

    if self.mask_i == 0 {
      write!(f, "{:#x}", 1 << self.mask_j)?;
    } else {
      write!(f, "0")?;
    }

    for i in 1..self.mask_size {
      if i == self.mask_i.into() {
        write!(f, ", {:#x}", 1 << self.mask_j)?;
      } else {
        write!(f, ", 0")?;
      }
    }

    write!(f, "}}")?;
    Ok(())
  }
}

// Formats masks of nodes.
pub struct NodeMask {
  mask_size: u16,
  components: Vec<u64>,
}

impl NodeMask {
  pub fn from_node(n: &Node, mask_size: u16) -> Self {
    Self {
      mask_size,
      components: n.mask.clone(),
    }
  }
}

impl Display for NodeMask {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.mask_size == 0 {
      return write!(f, "{{}}");
    }

    write!(f, "{{")?;
    write!(f, "{:#x}", self.components.get(0).unwrap_or(&0))?;

    for i in 1..self.mask_size {
      write!(f, ", {:#x}", self.components.get(i as usize).unwrap_or(&0))?;
    }

    write!(f, "}}")?;
    Ok(())
  }
}

impl<'a> Display for Header<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      concat!(
        "#ifndef VECS_VECS_H\n",
        "#define VECS_VECS_H\n",
        "\n",
        "#include <stdbool.h>\n",
        "#include <stddef.h>\n",
        "#include <stdint.h>\n",
        "\n",
      )
    )?;

    // Used in every SparseDynArray.
    DynArray::new("uint32_t".to_string()).header().fmt(f)?;

    for event in self.data.events.values() {
      // Event struct:
      let event_struct_name = EventStructName { name: event.name };

      write!(f, "// Event `{}`.\n\n", event.name)?;
      write!(f, "struct {} {{\n", event_struct_name)?;

      for field in event.fields.iter() {
        for typ_segment in field.typ.iter() {
          write!(f, "  {} ", typ_segment)?;
        }

        write!(f, "{};\n", field.name)?;
      }
      write!(f, "}};\n\n")?;

      // Event queue:
      let event_t = format!("struct {}", event_struct_name);
      DynQueue::new(event_t).header().fmt(f)?;
    }

    // Index and gen in one struct:
    write!(
      f,
      concat!(
        "struct vecs_id {{\n",
        "  uint32_t index;\n",
        "  uint32_t gen;\n",
        "}};\n",
      ),
    )?;

    // Used to access things from other things faster (index):
    let index_hash_array =
      HashDynArray::new("struct vecs_id".to_string(), "uint32_t".to_string());

    let index_hash_array_name = index_hash_array.get_name();
    index_hash_array.header().fmt(f)?;

    for component in self.data.components.values() {
      // Component struct:
      let component_name = component.name();
      let component_struct_name = ComponentStructName {
        name: component_name,
      };

      write!(f, "// Component `{}`.\n\n", component_name)?;
      write!(f, "struct {} {{\n", component_struct_name)?;

      for field in component.strukt.fields.iter() {
        for typ_segment in field.typ.iter() {
          write!(f, "  {} ", typ_segment)?;
        }

        write!(f, "{};\n", field.name)?;
      }
      write!(f, "}};\n\n")?;

      // Component mask:
      let component_mask_name = format!("vecs_component_{}_mask", component_name);

      write!(
        f,
        "static const uint64_t {}[{}] = {};\n\n",
        component_mask_name,
        self.data.node_mask_arr_size,
        ComponentMask::from_component(component, self.data.node_mask_arr_size),
      )?;

      // Component sparse array:
      let component_t = format!("struct {}", component_struct_name);
      DynArray::new(component_t.clone()).header().fmt(f)?;
      SparseDynArray::new(component_t.clone()).header().fmt(f)?;
    }

    for state in self.data.states.iter() {
      // State struct:
      let state_struct_name = StateStructName { name: state.name };

      write!(f, "// State `{}`.\n\n", state.name)?;
      write!(f, "struct {} {{\n", state_struct_name)?;

      for component in state.components.iter() {
        let component_struct_name = ComponentStructName {
          name: component.name,
        };

        let component_t = format!("struct {}", component_struct_name);
        let dyn_array = SparseDynArray::new(component_t.clone());
        let dyn_array_name = dyn_array.get_name();

        write!(f, "  struct {} {};\n", dyn_array_name, component.name)?;

        write!(
          f,
          "  struct {} entity_to_component_{};\n",
          index_hash_array_name, component.name
        )?;
      }
      write!(f, "}};\n\n")?;
    }

    // Union of all states:
    write!(f, "union vecs_state {{\n")?;
    for state in self.data.states.iter() {
      let state_struct_name = StateStructName { name: state.name };
      write!(f, "  struct {} {};\n", state_struct_name, state.name)?;
    }
    write!(f, "}};\n\n")?;

    for node in self.data.nodes.values() {
      // Node struct:
      let node_struct_name = NodeStructName { name: node.name };

      write!(f, "// Node `{}`.\n\n", node.name)?;
      write!(f, "struct {} {{\n", node_struct_name)?;

      for component in node.components.iter() {
        write!(f, "  uint32_t {}_index;\n", *component)?;
      }
      write!(f, "}};\n\n")?;

      // Node mask:
      let node_mask_name = format!("vecs_node_{}_mask", node.name);
      write!(
        f,
        "static const uint64_t {}[{}] = {};\n\n",
        node_mask_name,
        self.data.node_mask_arr_size,
        NodeMask::from_node(node, self.data.node_mask_arr_size),
      )?;

      // Array of node:
      let node_t = format!("struct {}", node_struct_name);
      DynArray::new(node_t).header().fmt(f)?;
    }

    // Entity struct:
    write!(
      f,
      concat!(
        "struct vecs_entity {{\n",
        "  uint64_t mask[{mask_size}];\n",
        "}};\n",
      ),
      mask_size = self.data.node_mask_arr_size,
    )?;

    let entity_t = "struct vecs_entity".to_string();
    DynArray::new(entity_t.clone()).header().fmt(f)?;
    let entity_array = SparseDynArray::new(entity_t);
    let entity_array_struct_name = entity_array.get_name();

    entity_array.header().fmt(f)?;

    // Main "engine" struct:

    write!(
      f,
      concat!(
        "// Engine.\n",
        "struct vecs_engine {{\n",
        "  struct {entity_array_struct_name} entities;\n",
        "  union vecs_state state;\n",
      ),
      entity_array_struct_name = entity_array_struct_name,
    )?;

    for node in self.data.nodes.values() {
      let node_struct_name = NodeStructName { name: node.name };
      let node_t = format!("struct {}", node_struct_name);
      let dyn_arr = DynArray::new(node_t);
      let dyn_arr_struct_name = dyn_arr.get_name();

      write!(f, "  struct {} nodes_{};\n", dyn_arr_struct_name, node.name)?;
      write!(
        f,
        "  struct {} entity_to_node_{};\n",
        index_hash_array_name, node.name,
      )?;
    }

    for event in self.data.events.values() {
      let event_struct_name = EventStructName { name: event.name };
      let event_t = format!("struct {}", event_struct_name);
      let dyn_queue = DynQueue::new(event_t);
      let dyn_queue_struct_name = dyn_queue.get_name();

      write!(
        f,
        "  struct {} events_{};\n",
        dyn_queue_struct_name, event.name,
      )?;
    }

    write!(f, "}};\n\n")?;

    // Component getters:
    for node in self.data.nodes.values() {
      for state in self.data.states.iter() {
        let node_struct_name = NodeStructName { name: node.name };

        for component in node.components.iter() {
          let component_struct_name = ComponentStructName { name: component };

          let component_t = format!("struct {}", component_struct_name);
          let component_array = SparseDynArray::new(component_t.clone());
          let component_array_name = component_array.get_name();

          write!(
            f,
            concat!(
              "static inline struct {component_struct_name} *vecs_state_{state_name}_node_{node_name}_get_{component_name}(struct vecs_engine *e, struct {node_struct_name} node) {{\n",
              "  return {component_array_method_get_unchecked}(&e->state.{state_name}.{component_name}, node.{component_name}_index);\n",
              "}}\n",
            ),
            component_struct_name = component_struct_name,
            state_name = state.name,
            node_name = node.name,
            component_name = component,
            node_struct_name = node_struct_name,
            component_array_method_get_unchecked =
              component_array_name.method("get_unchecked"),
          )?;
        }
      }
    }

    for system in self.data.systems.values() {
      // System function:
      let node_struct_name = NodeStructName { name: system.node };
      let event_struct_name = EventStructName { name: system.event };

      write!(f, "// System `{}`.\n", system.name)?;
      write!(
        f,
        "void {}(struct vecs_engine *engine, struct {} node, struct {} event);\n",
        system.name, node_struct_name, event_struct_name,
      )?;
    }

    // Engine methods:

    write!(
      f,
      "struct vecs_id vecs_add_entity(struct vecs_engine *e);\n",
    )?;

    for state in self.data.states.iter() {
      // Add components:
      for component in state.components.iter() {
        let component_struct_name = ComponentStructName {
          name: component.name,
        };

        write!(
          f,
          concat!(
            "struct vecs_id vecs_{state_name}_add_component_{component_name}(struct vecs_engine *e, struct vecs_id entity, struct {component_struct_name} component);\n",
            "bool vecs_{state_name}_remove_component_{component_name}(struct vecs_engine *e, struct vecs_id entity);\n",
            "void vecs_{state_name}_disable_component_{component_name}(struct vecs_engine *e, struct vecs_id entity);\n",
            "void vecs_{state_name}_enable_component_{component_name}(struct vecs_engine *e, struct vecs_id entity);\n\n",
          ),
          state_name = state.name,
          component_name = component.name,
          component_struct_name = component_struct_name,
        )?;
      }

      // State loops:
      write!(
        f,
        "void vecs_run_state_{}(struct vecs_engine *e);\n\n",
        state.name
      )?;
    }

    // Event emition:
    for event in self.data.events.values() {
      let event_struct_name = EventStructName { name: event.name };
      write!(
        f,
        "void vecs_emit_{}(struct vecs_engine *e, struct {} ev);\n",
        event.name, event_struct_name,
      )?;
    }

    write!(f, "\n#endif\n")?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::generate::header::ComponentMask;

  #[test]
  fn test_fmt_component_mask() {
    let fmter = ComponentMask {
      mask_size: 1,
      mask_i: 0,
      mask_j: 6,
    };
    let fmted = format!("{}", fmter);
    assert_eq!(fmted, "{0x40}");

    let fmter = ComponentMask {
      mask_size: 6,
      mask_i: 4,
      mask_j: 20,
    };
    let fmted = format!("{}", fmter);
    assert_eq!(fmted, "{0, 0, 0, 0, 0x100000, 0}");
  }
}
