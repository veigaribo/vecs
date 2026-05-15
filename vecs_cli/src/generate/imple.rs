use std::fmt::Display;

use crate::resolve::cst::Cst;

use super::{
  common::{ComponentStructName, EventStructName, NodeStructName},
  generics::{
    common::FunctionName, dyn_arrays::DynArray, dyn_queue::DynQueue,
    hash_dyn_arrays::HashDynArray, sparse_dyn_arrays::SparseDynArray,
  },
};

pub struct Impl<'a> {
  pub header_name: &'a str,
  pub data: &'a Cst<'a>,
}

impl<'a> Display for Impl<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "#include <stdlib.h>\n")?;
    write!(f, "#include <string.h>\n")?;
    write!(f, "#include \"{}\"\n\n", self.header_name)?;

    DynArray::new("uint32_t".to_string()).imple().fmt(f)?;

    for event in self.data.events.values() {
      let event_struct_name = EventStructName { name: event.name };
      let event_t = format!("struct {}", event_struct_name);
      DynQueue::new(event_t).imple().fmt(f)?;
    }

    let id_hash_fn_name = FunctionName::new("hash", vec!["struct vecs_id"]);

    write!(
      f,
      concat!(
        "// http://www.cse.yorku.ca/~oz/hash.html\n",
        "static inline uint32_t {hash_fn_name}(struct vecs_id key) {{\n",
        "  uint32_t hash = 5381;\n",
        "  /* hash * 33 + c */\n",
        "  for (uint32_t i = 0; i < sizeof(uint32_t) * 8; i += 8) {{\n",
        "    uint32_t c = (key.index >> i) & 0xff;\n",
        "    hash = ((hash << 5) + hash) + c;\n",
        "  }}\n",
        "  for (uint32_t i = 0; i < sizeof(uint32_t) * 8; i += 8) {{\n",
        "    uint32_t c = (key.gen >> i) & 0xff;\n",
        "    hash = ((hash << 5) + hash) + c;\n",
        "  }}\n",
        "  return hash;\n",
        "}}\n",
      ),
      hash_fn_name = id_hash_fn_name,
    )?;

    let id_eq_fn_name = FunctionName::new("eq", vec!["struct vecs_id"]);

    write!(
      f,
      concat!(
        "// http://www.cse.yorku.ca/~oz/hash.html\n",
        "static inline bool {eq_fn_name}(struct vecs_id a, struct vecs_id b) {{\n",
        "  return a.index == b.index && a.gen == b.gen;\n",
        "}}\n",
      ),
      eq_fn_name = id_eq_fn_name,
    )?;

    let index_hash_array =
      HashDynArray::new("struct vecs_id".to_string(), "uint32_t".to_string());

    let index_hash_array_name = index_hash_array.get_name();
    index_hash_array.imple().fmt(f)?;

    for component in self.data.components.values() {
      let component_name = component.name();
      let component_struct_name = ComponentStructName {
        name: component_name,
      };

      let component_t = format!("struct {}", component_struct_name);
      DynArray::new(component_t.clone()).imple().fmt(f)?;
      SparseDynArray::new(component_t.clone()).imple().fmt(f)?;
    }

    for node in self.data.nodes.values() {
      let node_struct_name = NodeStructName { name: node.name };
      let node_t = format!("struct {}", node_struct_name);
      DynArray::new(node_t).imple().fmt(f)?;
    }

    let entity_t = "struct vecs_entity".to_string();
    DynArray::new(entity_t.clone()).imple().fmt(f)?;
    let entity_array = SparseDynArray::new(entity_t);
    entity_array.imple().fmt(f)?;

    // Engine methods:

    let entity_array_struct_name = entity_array.get_name();

    write!(
      f,
      concat!(
        "struct vecs_id vecs_add_entity(struct vecs_engine *e) {{\n",
        "  struct vecs_entity ent = {{0}};\n",
        "  struct vecs_id id;\n",
        "  {entity_array_method_push}(&e->entities, ent, &id.index, &id.gen);\n",
        "  return id;\n",
        "}}\n",
      ),
      entity_array_method_push = entity_array_struct_name.method("push"),
    )?;

    // Mask utilities:
    write!(
      f,
      concat!(
        "static inline void mix_mask(const uint64_t *mask1, uint64_t *mask2) {{\n",
        "  for (size_t i = 0; i < {mask_size}; ++i) {{\n",
        "    mask2[i] |= mask1[i];\n",
        "  }}\n",
        "}}\n",
        "static inline void unmix_mask(const uint64_t *mask1, uint64_t *mask2) {{\n",
        "  for (size_t i = 0; i < {mask_size}; ++i) {{\n",
        "    mask2[i] &= ~mask1[i];\n",
        "  }}\n",
        "}}\n",
        "static inline bool match_mask(uint64_t *outer, const uint64_t *inner) {{\n",
        "  bool result = true;\n",
        "  for (size_t i = 0; i < {mask_size}; ++i) {{\n",
        "    result = result && ((inner[i] & outer[i]) == inner[i]);\n",
        "  }}\n",
        "  return result;\n",
        "}}\n",
      ),
      mask_size = self.data.node_mask_arr_size,
    )?;

    for state in self.data.states.iter() {
      for component in state.components.iter() {
        let component_struct_name = ComponentStructName {
          name: component.name,
        };

        let entity_array = SparseDynArray::new("struct vecs_entity".to_string());
        let entity_array_name = entity_array.get_name();

        let component_t = format!("struct {}", component_struct_name);
        let component_array = SparseDynArray::new(component_t.clone());
        let component_array_name = component_array.get_name();

        let component_mask_name = format!("vecs_component_{}_mask", component.name);

        // Add components:
        write!(
          f,
          concat!(
            "struct vecs_id vecs_{state_name}_add_component_{component_name}(struct vecs_engine *e, struct vecs_id entity, {component_t} component) {{\n",
            "  struct vecs_id component_id;\n",
            "  {component_array_method_push}(&e->state.{state_name}.{component_name}, component, &component_id.index, &component_id.gen);\n",
            "  {entity_to_component_array_method_add}(&e->state.{state_name}.entity_to_component_{component_name}, entity, component_id.index);\n",
            "\n",
            "  vecs_{state_name}_enable_component_{component_name}(e, entity);\n",
            "  return component_id;\n",
            "}}\n",
          ),
          state_name = state.name,
          component_name = component.name,
          component_t = component_t,
          component_array_method_push = component_array_name.method("push"),
          entity_to_component_array_method_add = index_hash_array_name.method("add"),
        )?;

        // Remove components:
        write!(
          f,
          concat!(
            "bool vecs_{state_name}_remove_component_{component_name}(struct vecs_engine *e, struct vecs_id entity) {{\n",
            "  vecs_{state_name}_disable_component_{component_name}(e, entity);\n",
            "\n",
            "  struct vecs_id component_id;\n",
            "  bool found = {entity_to_component_array_method_remove}(&e->state.{state_name}.entity_to_component_{component_name}, entity, &component_id.index);\n",
            "  if (!found)\n",
            "    return false;\n",
            "  {component_array_method_remove}(&e->state.{state_name}.{component_name}, component_id.index);\n",
            "}}\n",
          ),
          state_name = state.name,
          component_name = component.name,
          component_array_method_remove = component_array_name.method("remove"),
          entity_to_component_array_method_remove =
            index_hash_array_name.method("remove"),
        )?;

        // Disable components:
        write!(
          f,
          concat!(
            "void vecs_{state_name}_disable_component_{component_name}(struct vecs_engine *e, struct vecs_id entity) {{\n",
            "  struct vecs_entity *ent = {entity_array_method_get}(&e->entities, entity.index, entity.gen);\n",
          ),
          state_name = state.name,
          entity_array_method_get = entity_array_name.method("get"),
          component_name = component.name,
        )?;

        for node in self.data.nodes.values() {
          if node.components.contains(component.name) {
            let node_struct_name = NodeStructName { name: node.name };
            let node_mask_name = format!("vecs_node_{}_mask", node.name);

            let node_t = format!("struct {}", node_struct_name);
            let node_array = DynArray::new(node_t);
            let node_array_struct_name = node_array.get_name();

            write!(
              f,
              concat!(
                "  if (match_mask(ent->mask, {node_mask_name})) {{\n",
                "    uint32_t node_index;\n",
                "    {entity_to_node_method_remove}(&e->entity_to_node_{node_name}, entity, &node_index);\n",
                "    {node_array_method_remove}(&e->nodes_{node_name}, node_index);\n",
                "  }}\n",
              ),
              node_mask_name = node_mask_name,
              node_name = node.name,
              node_array_method_remove = node_array_struct_name.method("swap_remove"),
              entity_to_node_method_remove = index_hash_array_name.method("remove"),
            )?;
          }
        }

        write!(
          f,
          concat!("  unmix_mask({component_mask_name}, ent->mask);\n", "}}\n",),
          component_mask_name = component_mask_name,
        )?;

        // Enable components:
        write!(
          f,
          concat!(
            "void vecs_{state_name}_enable_component_{component_name}(struct vecs_engine *e, struct vecs_id entity) {{\n",
            "  struct vecs_entity *ent = {entity_array_method_get}(&e->entities, entity.index, entity.gen);\n",
            "  mix_mask({component_mask_name}, ent->mask);\n",
          ),
          state_name = state.name,
          entity_array_method_get = entity_array_name.method("get"),
          component_name = component.name,
          component_mask_name = component_mask_name,
        )?;

        for node in self.data.nodes.values() {
          if node.components.contains(component.name) {
            let node_struct_name = NodeStructName { name: node.name };
            let node_mask_name = format!("vecs_node_{}_mask", node.name);

            write!(
              f,
              concat!(
                "  if (match_mask(ent->mask, {node_mask_name})) {{\n",
                "    uint32_t component_index;\n",
                "    struct {node_struct_name} node;\n",
              ),
              node_mask_name = node_mask_name,
              node_struct_name = node_struct_name,
            )?;

            for node_component in node.components.iter() {
              write!(
                f,
                concat!(
                  "    {entity_to_component_array_method_get}(&e->state.{state_name}.entity_to_component_{component_name}, entity, &component_index);\n",
                  "    node.{component_name}_index = component_index;\n",
                ),
                component_name = node_component,
                state_name = state.name,
                entity_to_component_array_method_get =
                  index_hash_array_name.method("get"),
              )?;
            }

            let node_t = format!("struct {}", node_struct_name);
            let node_array = DynArray::new(node_t);
            let node_array_struct_name = node_array.get_name();

            write!(
              f,
              concat!(
                "    uint32_t node_index = {node_array_method_push}(&e->nodes_{node_name}, node);\n",
                "    {entity_to_node_method_add}(&e->entity_to_node_{node_name}, entity, node_index);\n",
                "  }}\n",
              ),
              node_name = node.name,
              node_array_method_push = node_array_struct_name.method("push"),
              entity_to_node_method_add = index_hash_array_name.method("add"),
            )?;
          }
        }

        write!(f, concat!("}}\n",),)?;
      }

      // State loops:
      write!(
        f,
        "void vecs_run_state_{}(struct vecs_engine *e) {{\n",
        state.name
      )?;

      for event in self.data.events.values() {
        let event_struct_name = EventStructName { name: event.name };
        let event_t = format!("struct {}", event_struct_name);
        let event_queue = DynQueue::new(event_t.clone());
        let event_queue_name = event_queue.get_name();

        write!(f, "  while (e->events_{}.len > 0) {{\n", event.name)?;
        write!(
          f,
          "    {} ev = {}(&e->events_{});\n",
          event_t,
          event_queue_name.method("dequeue"),
          event.name,
        )?;

        for system_layer in state.systems.iter() {
          for system_name in system_layer {
            let system = self
              .data
              .systems
              .get(system_name)
              .expect("failed to find system in state");

            if system.event == event.name {
              let node_struct_name = NodeStructName { name: system.node };

              write!(
                f,
                concat!(
                  "    for (size_t i = 0; i < e->nodes_{node_name}.len; ++i) {{\n",
                  "      struct {node_struct_name} *node = &e->nodes_{node_name}.items[i];\n",
                  "      {system_name}(e, *node, ev);\n",
                  "    }}\n",
                ),
                node_name = system.node,
                node_struct_name = node_struct_name,
                system_name = system.name,
              )?;
            }
          }
        }
        write!(f, "  }}\n")?;
      }

      write!(f, "}}\n")?;
    }

    // Event emition:
    for event in self.data.events.values() {
      let event_struct_name = EventStructName { name: event.name };
      let event_queue = DynQueue::new(format!("struct {}", event_struct_name));
      let event_queue_name = event_queue.get_name();

      write!(
        f,
        concat!(
          "void vecs_emit_{event_name}(struct vecs_engine *e, struct {event_struct_name} ev) {{\n",
          "  {event_queue_method_enqueue}(&e->events_{event_name}, ev);\n",
          "}}\n",
        ),
        event_name = event.name,
        event_struct_name = event_struct_name,
        event_queue_method_enqueue = event_queue_name.method("enqueue"),
      )?;
    }

    Ok(())
  }
}
