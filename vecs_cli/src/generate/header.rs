use std::fmt::Display;

use crate::{
  generate::{
    common::{
      ComponentOpAddStructName, ComponentOpAddTmpStructName,
      ComponentOpUpdateStructName, ComponentTmpOps,
    },
    generics::skip_lists::SkipList,
  },
  resolve::cst::Cst,
};

use super::{
  common::{ComponentStructName, EventStructName, NodeStructName},
  constants::{
    ComponentMask, ComponentMaskName, NodeMask, NodeMaskName, StateIdName,
  },
  generics::{
    common::{method_name, whatever_name},
    dyn_arrays::DynArray,
    dyn_queue::DynQueue,
    sparse_dyn_arrays::SparseDynArray,
  },
};

fn write_iterator<W: std::fmt::Write, I: Iterator<Item = T>, T: Display>(
  w: &mut W,
  mut iter: I,
) -> std::fmt::Result {
  let next = iter.next();

  match next {
    Some(head) => {
      write!(w, "{}", head)?;
    }
    None => return Ok(()),
  }

  for i in iter {
    write!(w, " {}", i)?;
  }

  return Ok(());
}

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

    for include in self.data.includes.iter() {
      write!(f, "#include {}\n", include)?;
    }
    write!(f, "\n")?;

    write!(
      f,
      concat!(
        "// Index and gen in one struct. Used for permanent storage of entities and\n",
        "// component references.\n",
        "typedef struct vecs_id {{\n",
        "  uint32_t index;\n",
        "  uint32_t gen;\n",
        "}} vecs_id_t;\n",
        "\n",
        "// Used to temporarily refer to entities and components that have not yet\n",
        "// been persisted, and so do not yet posses a permanent `vecs_id_t`.\n",
        "typedef struct vecs_tmp_id {{\n",
        "  uint32_t index;\n",
        "}} vecs_tmp_id_t;\n",
        "\n",
        "// TODO: Remove this.\n",
        "typedef struct vecs_frame {{\n",
        "  float delta;\n",
        "  double runtime;\n",
        "  uint64_t frame;\n",
        "}} vecs_frame_t;\n",
      ),
    )?;

    // Used in every SparseDynArray.
    DynArray::new("uint32_t").header().fmt(f)?;
    DynArray::new("uint64_t").header().fmt(f)?;
    DynQueue::new("uint32_t").header().fmt(f)?;

    for event in self.data.events.values() {
      // Event struct:
      let event_t = EventStructName::new(event.name);

      write!(f, "// Event `{}`.\ntypedef ", event.name,)?;

      if !event.is_empty() {
        let event_type_name_iter = event.type_components.iter();
        write_iterator(f, event_type_name_iter)?;
      } else {
        write!(f, "struct {{}}")?;
      }

      write!(f, " {};\n\n", event_t)?;

      // Event queue:
      DynQueue::new(event_t).header().fmt(f)?;
    }

    // Used to access things from other things faster (index):
    let index_index = SkipList::new("vecs_id_t", "uint32_t");

    let index_index_t = index_index.get_type();
    index_index.header().fmt(f)?;

    write!(
      f,
      concat!(
        "typedef struct vecs_op_store_entity {{\n",
        "  vecs_tmp_id_t tmp_entity;\n",
        "  ptrdiff_t location_offset;\n",
        "}} vecs_op_store_entity_t;\n",
        "typedef struct vecs_op_store_component {{\n",
        "  vecs_tmp_id_t tmp_component;\n",
        "  ptrdiff_t location_offset;\n",
        "}} vecs_op_store_component_t;\n",
        "typedef struct vecs_op_enable_component {{\n",
        "  vecs_id_t entity;\n",
        "}} vecs_op_enable_component_t;\n",
        "typedef struct vecs_op_remove_component {{\n",
        "  vecs_id_t entity;\n",
        "}} vecs_op_remove_component_t;\n",
        "typedef struct vecs_op_disable_component {{\n",
        "  vecs_id_t entity;\n",
        "}} vecs_op_disable_component_t;\n\n",
      ),
    )?;

    for component in self.data.components.values() {
      // Component struct:
      let component_name = component.name();
      let component_t = ComponentStructName::new(component_name);

      write!(f, "// Component `{}`.\ntypedef ", component_name)?;

      if !component.is_empty() {
        let component_type_name_iter = component.typ.type_components.iter();
        write_iterator(f, component_type_name_iter)?;
      } else {
        write!(f, "struct {{}}")?;
      }

      write!(f, " {};\n\n", component_t)?;

      // Component mask:
      write!(
        f,
        "static const uint64_t {}[{}] = {};\n\n",
        ComponentMaskName::new(component_name),
        self.data.node_mask_arr_size,
        ComponentMask::from_component(component, self.data.node_mask_arr_size),
      )?;

      if !component.is_empty() {
        // Component sparse array:
        DynArray::new(component_t.clone()).header().fmt(f)?;
        SparseDynArray::new(component_t.clone()).header().fmt(f)?;
      }

      // Temporary component operations:
      let ops = ComponentTmpOps::new(component_name);

      if !component.is_empty() {
        write!(
          f,
          concat!(
            "typedef struct vecs_op_add_component_{component_name} {{\n",
            "  vecs_id_t entity;\n",
            "  {component_t} component;\n",
            "}} {component_add_t};\n",
            "typedef struct vecs_op_tmp_add_component_{component_name} {{\n",
            "  vecs_tmp_id_t tmp_entity;\n",
            "  {component_t} component;\n",
            "}} {component_add_tmp_t};\n",
            "typedef struct vecs_op_update_component_{component_name} {{\n",
            "  vecs_id_t entity;\n",
            "  {component_t} component;\n",
            "}} {component_update_t};\n",
          ),
          component_name = component_name,
          component_t = component_t,
          component_add_t = ops.add_t,
          component_add_tmp_t = ops.add_tmp_t,
          component_update_t = ops.update_t,
        )?;
      } else {
        write!(
          f,
          concat!(
            "typedef struct vecs_op_add_component_{component_name} {{\n",
            "  vecs_id_t entity;\n",
            "}} {component_add_t};\n",
            "typedef struct vecs_op_tmp_add_component_{component_name} {{\n",
            "  vecs_tmp_id_t tmp_entity;\n",
            "}} {component_add_tmp_t};\n",
          ),
          component_name = component_name,
          component_add_t = ops.add_t,
          component_add_tmp_t = ops.add_tmp_t,
        )?;
      }
    }

    // State enum:
    write!(f, "typedef enum vecs_state {{\n",)?;
    write!(f, "  VECS_STATE_NONE,\n",)?;
    for state in self.data.states.values() {
      let state_id = StateIdName::new(state.name);
      write!(f, "  {},\n", state_id)?;
    }
    write!(f, "}} vecs_state_t;\n\n",)?;

    // Forward declare the engine struct so the operations can refer to it
    write!(f, "struct vecs_engine;\n\n",)?;

    // Deferred operation structs:

    // These operations should be applied first
    write!(
      f,
      concat!(
        "typedef struct vecs_op_union_add_component {{\n",
        "  vecs_id_t (*apply)(struct vecs_engine *, vecs_id_t *new_entities, struct vecs_op_union_add_component);\n",
        "  union {{\n",
      )
    )?;

    for component in self.data.components.values() {
      let component_name = component.name();
      let add_t = ComponentOpAddStructName::new(component_name);
      let add_tmp_t = ComponentOpAddTmpStructName::new(component_name);

      write!(
        f,
        concat!(
          "    {add_t} add_{component_name};\n",
          "    {add_tmp_t} add_tmp_{component_name};\n",
        ),
        component_name = component_name,
        add_t = add_t,
        add_tmp_t = add_tmp_t,
      )?;
    }
    write!(
      f,
      concat!("  }};\n", "}} vecs_op_union_add_component_t;\n\n")
    )?;

    // These operations should be applied after start and before end, in no specified
    // order
    write!(
      f,
      concat!(
        "typedef struct vecs_op_union_other {{\n",
        "  void (*apply)(struct vecs_engine *, vecs_id_t *restrict new_entities, vecs_id_t *restrict new_components, struct vecs_op_union_other);\n",
        "  union {{\n",
        "    vecs_op_store_entity_t store_entity;\n",
        "    vecs_op_store_component_t store_component;\n",
        "    vecs_op_enable_component_t enable;\n",
        "    vecs_op_disable_component_t disable;\n",
      )
    )?;

    for component in self.data.components.values() {
      let component_name = component.name();

      if !component.is_empty() {
        let update_t = ComponentOpUpdateStructName::new(component_name);

        write!(
          f,
          concat!("    {update_t} update_{component_name};\n",),
          component_name = component_name,
          update_t = update_t,
        )?;
      }
    }
    write!(f, concat!("  }};\n", "}} vecs_op_union_other_t;\n\n"))?;

    // These operations should be applied last
    write!(
      f,
      concat!(
        "typedef struct vecs_op_union_remove_component {{\n",
        "  void (*apply)(struct vecs_engine *, struct vecs_op_union_remove_component);\n",
        "  union {{\n",
        "    vecs_op_remove_component_t remove;\n",
        "  }};\n",
        "}} vecs_op_union_remove_component_t;\n\n"
      )
    )?;

    let op_add_component_queue = DynQueue::new("vecs_op_union_add_component_t");
    let op_add_component_queue_t = op_add_component_queue.get_type();
    op_add_component_queue.header().fmt(f)?;

    let op_other_queue = DynQueue::new("vecs_op_union_other_t");
    let op_other_queue_t = op_other_queue.get_type();
    op_other_queue.header().fmt(f)?;

    let op_remove_component_queue = DynQueue::new("vecs_op_union_remove_component_t");
    let op_remove_component_queue_t = op_remove_component_queue.get_type();
    op_remove_component_queue.header().fmt(f)?;

    for node in self.data.nodes.values() {
      // Node struct:
      let node_t = NodeStructName::new(node.name);

      // TODO: Add entity ID
      write!(f, "// Node `{}`.\n\n", node.name)?;
      write!(
        f,
        "typedef struct {} {{\n",
        whatever_name!("node", node.name),
      )?;

      for component_name in node.components.iter() {
        let component = self
          .data
          .components
          .get(component_name)
          .expect("component not found");

        if !component.is_empty() {
          write!(f, "  uint32_t {}_index;\n", *component_name)?;
        }
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

    // Global struct:

    write!(
      f,
      concat!("// Globals.\n", "typedef struct vecs_globals {{\n",),
    )?;

    for global in self.data.globals.values() {
      write!(f, "  ")?;
      let event_type_name_iter = global.type_components.iter();
      write_iterator(f, event_type_name_iter)?;
      write!(f, " {};\n", global.name)?;
    }

    write!(f, "}} vecs_globals_t;\n\n")?;

    // Main "engine" struct:

    write!(
      f,
      concat!(
        "// Engine.\n",
        "typedef struct vecs_engine {{\n",
        "  vecs_state_t state;\n",
        "  vecs_globals_t globals;\n",
        "\n",
        "  // Deferred operations\n",
        "  uint32_t entities_to_add;\n",
        "  vecs_state_t next_state;\n",
        "  {op_add_component_queue_t} ops_add_component;\n",
        "  {op_remove_component_queue_t} ops_remove_component;\n",
        "  {op_other_queue_t} ops_other;\n",
        "\n",
        "  {entity_array_t} entities;\n",
      ),
      entity_array_t = entity_array_t,
      op_add_component_queue_t = op_add_component_queue_t,
      op_other_queue_t = op_other_queue_t,
      op_remove_component_queue_t = op_remove_component_queue_t,
    )?;

    for component in self.data.components.values() {
      if !component.is_empty() {
        let component_name = component.name();
        let component_t = ComponentStructName::new(component_name);

        let dyn_array = SparseDynArray::new(component_t.clone());
        let dyn_array_t = dyn_array.get_type();

        write!(f, "  {} components_{};\n", dyn_array_t, component_name)?;
        write!(
          f,
          "  {} entity_to_component_{};\n",
          index_index_t, component_name,
        )?;
      }
    }

    for node in self.data.nodes.values() {
      let node_t = NodeStructName::new(node.name);
      let dyn_array = DynArray::new(node_t);
      let dyn_array_t = dyn_array.get_type();

      write!(f, "  {} nodes_{};\n", dyn_array_t, node.name)?;
      write!(f, "  {} entity_to_node_{};\n", index_index_t, node.name,)?;
    }

    for event in self.data.events.values() {
      let event_t = EventStructName::new(event.name);
      let dyn_queue = DynQueue::new(event_t);
      let dyn_queue_t = dyn_queue.get_type();

      write!(f, "  {} events_{};\n", dyn_queue_t, event.name)?;
    }

    write!(f, "}} vecs_engine_t;\n\n")?;

    // Component getters:
    for node in self.data.nodes.values() {
      let node_t = NodeStructName::new(node.name);

      for component_name in node.components.iter() {
        let component = self
          .data
          .components
          .get(component_name)
          .expect("component not found");

        if !component.is_empty() {
          let component_t = ComponentStructName::new(component_name);
          let component_array = SparseDynArray::new(component_t.clone());
          let component_array_t = component_array.get_type();

          write!(
            f,
            concat!(
              "static inline {component_t} *vecs_node_{node_name}_get_{component_name}(vecs_engine_t *e, {node_t} node) {{\n",
              "  return {component_array_method_get_unchecked}(&e->components_{component_name}, node.{component_name}_index);\n",
              "}}\n",
            ),
            component_t = component_t,
            node_name = node.name,
            component_name = component_name,
            node_t = node_t,
            component_array_method_get_unchecked =
              method_name!(&component_array_t, "get_unchecked"),
          )?;
        }
      }
    }

    for system in self.data.systems.values() {
      // System function:
      let event_t = EventStructName::new(system.event);

      if let Some(node) = system.node {
        let node_t = NodeStructName::new(node);

        write!(
          f,
          concat!(
            "// System `{system_name}`.\n",
            "void {system_name}(vecs_engine_t *engine, {node_t} node, {event_t} event);\n"
          ),
          system_name = system.name,
          node_t = node_t,
          event_t = event_t,
        )?;
      } else {
        write!(
          f,
          concat!(
            "// System singleton `{system_name}`.\n",
            "void {system_name}(vecs_engine_t *engine, {event_t} event);\n"
          ),
          system_name = system.name,
          event_t = event_t,
        )?;
      }
    }

    // Engine methods:

    write!(
      f,
      concat!(
        "void vecs_init(vecs_engine_t *e);\n",
        "void vecs_destroy(vecs_engine_t *e);\n",
        "vecs_id_t vecs_add_entity(vecs_engine_t *e);\n",
        "vecs_tmp_id_t vecs_schedule_add_entity(vecs_engine_t *e);\n",
      ),
    )?;

    // Component manipulation:
    for component in self.data.components.values() {
      let component_name = component.name();
      let component_t = ComponentStructName::new(component_name);

      for state in self.data.states.values() {
        // Immediate methods
        write!(
          f,
          concat!(
            "bool vecs_has_component_{component_name}(vecs_engine_t *e, vecs_id_t entity);\n",
            "void vecs_{state_name}_disable_component_{component_name}(vecs_engine_t *e, vecs_id_t entity);\n",
            "void vecs_{state_name}_enable_component_{component_name}(vecs_engine_t *e, vecs_id_t entity);\n",
          ),
          state_name = state.name,
          component_name = component_name,
        )?;

        if !component.is_empty() {
          write!(
            f,
            concat!(
              "vecs_id_t vecs_{state_name}_add_component_{component_name}(vecs_engine_t *e, vecs_id_t entity, {component_t} component);\n",
              "vecs_id_t vecs_{state_name}_upsert_component_{component_name}(vecs_engine_t *e, vecs_id_t entity, {component_t} component);\n",
              "vecs_id_t vecs_{state_name}_update_component_{component_name}(vecs_engine_t *e, vecs_id_t entity, {component_t} component);\n",
              "bool vecs_{state_name}_remove_component_{component_name}(vecs_engine_t *e, vecs_id_t entity);\n",
            ),
            state_name = state.name,
            component_name = component_name,
            component_t = component_t,
          )?;
        } else {
          // Component is empty
          write!(
            f,
            concat!(
              "void vecs_{state_name}_add_component_{component_name}(vecs_engine_t *e, vecs_id_t entity);\n",
              "void vecs_{state_name}_remove_component_{component_name}(vecs_engine_t *e, vecs_id_t entity);\n",
            ),
            state_name = state.name,
            component_name = component_name,
          )?;
        }
      }

      // Deferred methods
      write!(
        f,
        concat!(
          "void vecs_schedule_store_entity_in_{component_name}(vecs_engine_t *e, vecs_tmp_id_t tmp_entity, vecs_id_t *location);\n",
          "void vecs_schedule_store_component_{component_name}(vecs_engine_t *e, vecs_tmp_id_t tmp_component, vecs_id_t *location);\n",
          "void vecs_schedule_remove_component_{component_name}(vecs_engine_t *e, vecs_id_t entity);\n",
          "void vecs_schedule_disable_component_{component_name}(vecs_engine_t *e, vecs_id_t entity);\n",
          "void vecs_schedule_enable_component_{component_name}(vecs_engine_t *e, vecs_id_t entity);\n",
        ),
        component_name = component_name,
      )?;

      if !component.is_empty() {
        write!(
          f,
          concat!(
            "vecs_tmp_id_t vecs_schedule_add_component_{component_name}(vecs_engine_t *e, vecs_id_t entity, {component_t} component);\n",
            "vecs_tmp_id_t vecs_schedule_tmp_add_component_{component_name}(vecs_engine_t *e, vecs_tmp_id_t entity, {component_t} component);\n",
            "void vecs_schedule_upsert_component_{component_name}(vecs_engine_t *e, vecs_id_t entity, {component_t} component);\n",
            "void vecs_schedule_update_component_{component_name}(vecs_engine_t *e, vecs_id_t entity, {component_t} component);\n",
          ),
          component_name = component_name,
          component_t = component_t,
        )?;
      } else {
        // Component is empty
        write!(
          f,
          concat!(
            "void vecs_schedule_add_component_{component_name}(vecs_engine_t *e, vecs_id_t entity);\n",
            "void vecs_schedule_tmp_add_component_{component_name}(vecs_engine_t *e, vecs_tmp_id_t entity);\n",
          ),
          component_name = component_name,
        )?;
      }
    }

    // Node getters:
    for node in self.data.nodes.values() {
      let node_t = NodeStructName::new(node.name);
      let dyn_arr = DynArray::new(node_t);
      let dyn_arr_t = dyn_arr.get_type();

      write!(
        f,
        "typedef {} vecs_node_{}_array_t;\n",
        dyn_arr_t, node.name
      )?;

      write!(
        f,
        "vecs_node_{node_name}_array_t vecs_nodes_{node_name}(vecs_engine_t *e);\n",
        node_name = node.name,
      )?;
    }

    for state in self.data.states.values() {
      write!(
        f,
        "void vecs_schedule_state_to_{}(vecs_engine_t *e);\n",
        state.name,
      )?;

      // State transitions:
      for other_state in self.data.states.values() {
        if state.name == other_state.name {
          continue;
        }

        write!(
          f,
          "void vecs_state_{}_to_{}(vecs_engine_t *e);\n",
          other_state.name, state.name,
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
