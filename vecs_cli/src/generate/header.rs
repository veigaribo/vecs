use std::fmt::Display;

use crate::resolve::cst::Cst;

use super::{
  common::{ComponentStructName, EventStructName, NodeStructName, StateStructName},
  generics::{
    common::{method_name, whatever_name},
    dyn_arrays::DynArray,
    dyn_queue::DynQueue,
    hash_dyn_arrays::HashDynArray,
    sparse_dyn_arrays::SparseDynArray,
  },
  masks::{ComponentMask, ComponentMaskName, NodeMask, NodeMaskName},
};

pub struct Header<'a> {
  pub data: &'a Cst<'a>,
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
    DynArray::new("uint32_t").header().fmt(f)?;

    for event in self.data.events.values() {
      // Event struct:
      let event_t = EventStructName::new(event.name);

      write!(f, "// Event `{}`.\n\n", event.name)?;
      write!(
        f,
        "typedef struct {} {{\n",
        whatever_name!("event", event.name),
      )?;

      for field in event.fields.iter() {
        for typ_segment in field.typ.iter() {
          write!(f, "  {} ", typ_segment)?;
        }

        write!(f, "{};\n", field.name)?;
      }
      write!(f, "}} {};\n\n", event_t)?;

      // Event queue:
      DynQueue::new(event_t).header().fmt(f)?;
    }

    // Index and gen in one struct:
    write!(
      f,
      concat!(
        "typedef struct {whatever} {{\n",
        "  uint32_t index;\n",
        "  uint32_t gen;\n",
        "}} vecs_id_t;\n",
      ),
      whatever = whatever_name!("vecs_id"),
    )?;

    // Used to access things from other things faster (index):
    let index_hash_array = HashDynArray::new("vecs_id_t", "uint32_t");

    let index_hash_array_t = index_hash_array.get_type();
    index_hash_array.header().fmt(f)?;

    for component in self.data.components.values() {
      // Component struct:
      let component_name = component.name();
      let component_t = ComponentStructName::new(component_name);

      write!(f, "// Component `{}`.\n\n", component_name)?;
      write!(
        f,
        "typedef struct {} {{\n",
        whatever_name!("component", component_name),
      )?;

      for field in component.strukt.fields.iter() {
        for typ_segment in field.typ.iter() {
          write!(f, "  {} ", typ_segment)?;
        }

        write!(f, "{};\n", field.name)?;
      }
      write!(f, "}} {};\n\n", component_t)?;

      // Component mask:
      write!(
        f,
        "static const uint64_t {}[{}] = {};\n\n",
        ComponentMaskName::new(component_name),
        self.data.node_mask_arr_size,
        ComponentMask::from_component(component, self.data.node_mask_arr_size),
      )?;

      // Component sparse array:
      DynArray::new(component_t.clone()).header().fmt(f)?;
      SparseDynArray::new(component_t.clone()).header().fmt(f)?;
    }

    for state in self.data.states.iter() {
      // State struct:
      let state_t = StateStructName::new(state.name);

      write!(f, "// State `{}`.\n\n", state.name)?;
      write!(
        f,
        "typedef struct {} {{\n",
        whatever_name!("state", state.name),
      )?;

      for component in state.components.iter() {
        let component_t = ComponentStructName::new(component.name);

        let dyn_array = SparseDynArray::new(component_t.clone());
        let dyn_array_t = dyn_array.get_type();

        write!(f, "  {} {};\n", dyn_array_t, component.name)?;

        write!(
          f,
          "  {} entity_to_component_{};\n",
          index_hash_array_t, component.name,
        )?;
      }
      write!(f, "}} {};\n\n", state_t)?;
    }

    // Union of all states:
    write!(f, "typedef union vecs_state {{\n")?;
    for state in self.data.states.iter() {
      let state_t = StateStructName::new(state.name);
      write!(f, "  {} {};\n", state_t, state.name)?;
    }
    write!(f, "}} vecs_state_t;\n\n")?;

    for node in self.data.nodes.values() {
      // Node struct:
      let node_t = NodeStructName::new(node.name);

      write!(f, "// Node `{}`.\n\n", node.name)?;
      write!(
        f,
        "typedef struct {} {{\n",
        whatever_name!("node", node.name),
      )?;

      for component in node.components.iter() {
        write!(f, "  uint32_t {}_index;\n", *component)?;
      }
      write!(f, "}} {};\n\n", node_t)?;

      // Node mask:
      write!(
        f,
        "static const uint64_t {}[{}] = {};\n\n",
        NodeMaskName::new(node.name),
        self.data.node_mask_arr_size,
        NodeMask::from_node(node, self.data.node_mask_arr_size),
      )?;

      // Array of node:
      DynArray::new(node_t).header().fmt(f)?;
    }

    // Entity struct:
    write!(
      f,
      concat!(
        "typedef struct vecs_entity {{\n",
        "  uint64_t mask[{mask_size}];\n",
        "}} vecs_entity_t;\n",
      ),
      mask_size = self.data.node_mask_arr_size,
    )?;

    DynArray::new("vecs_entity_t").header().fmt(f)?;
    let entity_array = SparseDynArray::new("vecs_entity_t");
    let entity_array_t = entity_array.get_type();

    entity_array.header().fmt(f)?;

    // Main "engine" struct:

    write!(
      f,
      concat!(
        "// Engine.\n",
        "typedef struct vecs_engine {{\n",
        "  {entity_array_t} entities;\n",
        "  vecs_state_t state;\n",
      ),
      entity_array_t = entity_array_t,
    )?;

    for node in self.data.nodes.values() {
      let node_t = NodeStructName::new(node.name);
      let dyn_arr = DynArray::new(node_t);
      let dyn_arr_t = dyn_arr.get_type();

      write!(f, "  {} nodes_{};\n", dyn_arr_t, node.name)?;
      write!(
        f,
        "  {} entity_to_node_{};\n",
        index_hash_array_t, node.name,
      )?;
    }

    for event in self.data.events.values() {
      let event_t = EventStructName::new(event.name);
      let dyn_queue = DynQueue::new(event_t);
      let dyn_queue_t = dyn_queue.get_type();

      write!(f, "  {} events_{};\n", dyn_queue_t, event.name,)?;
    }

    write!(f, "}} vecs_engine_t;\n\n")?;

    // Component getters:
    for node in self.data.nodes.values() {
      for state in self.data.states.iter() {
        let node_t = NodeStructName::new(node.name);

        for component in node.components.iter() {
          let component_t = ComponentStructName::new(component);
          let component_array = SparseDynArray::new(component_t.clone());
          let component_array_t = component_array.get_type();

          write!(
            f,
            concat!(
              "static inline {component_t} *vecs_state_{state_name}_node_{node_name}_get_{component_name}(vecs_engine_t *e, {node_t} node) {{\n",
              "  return {component_array_method_get_unchecked}(&e->state.{state_name}.{component_name}, node.{component_name}_index);\n",
              "}}\n",
            ),
            component_t = component_t,
            state_name = state.name,
            node_name = node.name,
            component_name = component,
            node_t = node_t,
            component_array_method_get_unchecked =
              method_name!(&component_array_t, "get_unchecked"),
          )?;
        }
      }
    }

    for system in self.data.systems.values() {
      // System function:
      let node_t = NodeStructName::new(system.node);
      let event_t = EventStructName::new(system.event);

      write!(f, "// System `{}`.\n", system.name)?;
      write!(
        f,
        "void {}(vecs_engine_t *engine, {} node, {} event);\n",
        system.name, node_t, event_t,
      )?;
    }

    // Engine methods:

    write!(f, "vecs_id_t vecs_add_entity(vecs_engine_t *e);\n",)?;

    for state in self.data.states.iter() {
      // Add components:
      for component in state.components.iter() {
        let component_t = ComponentStructName::new(component.name);

        write!(
          f,
          concat!(
            "vecs_id_t vecs_{state_name}_add_component_{component_name}(vecs_engine_t *e, vecs_id_t entity, {component_t} component);\n",
            "bool vecs_{state_name}_remove_component_{component_name}(vecs_engine_t *e, vecs_id_t entity);\n",
            "void vecs_{state_name}_disable_component_{component_name}(vecs_engine_t *e, vecs_id_t entity);\n",
            "void vecs_{state_name}_enable_component_{component_name}(vecs_engine_t *e, vecs_id_t entity);\n\n",
          ),
          state_name = state.name,
          component_name = component.name,
          component_t = component_t,
        )?;
      }

      // State loops:
      write!(
        f,
        "void vecs_run_state_{}(vecs_engine_t *e);\n\n",
        state.name,
      )?;
    }

    // Event emition:
    for event in self.data.events.values() {
      let event_t = EventStructName::new(event.name);
      write!(
        f,
        "void vecs_emit_{}(vecs_engine_t *e, {} ev);\n",
        event.name, event_t,
      )?;
    }

    write!(f, "\n#endif\n")?;

    Ok(())
  }
}
